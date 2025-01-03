use std::io::Read;
use std::path::{Path, PathBuf};

use campaign::{Campaign, FileMeta, ItemRegistry};
use chrono::format::Item;
use clap::Parser;

mod apis;
mod campaign;
mod data;
mod meta;
mod registry;
mod ui;

mod state;

use data::character::{Character, FileCharacter};
use data::shop::FileShop;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use registry::ItemType;
use serde::Deserialize;
use ui::translator::EngNerdI18n;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    campaign_file: Option<String>,
}

fn load_object_vector<T: for<'a> Deserialize<'a>>(folder: PathBuf) -> anyhow::Result<Vec<T>> {
    let entries = std::fs::read_dir(folder)?;

    let objects = entries
        .filter_map(|maybe_entry| 
            match maybe_entry {
                Ok(entry) => load_object(&entry.path()).ok(),
                Err(_) => None
            })
        .collect();

    Ok(objects)
}

fn load_object<T: for<'a> Deserialize<'a>>(path: &PathBuf) -> anyhow::Result<T>{
    let file = std::fs::File::open(path)
        .inspect_err(|e| 
            log::warn!("Could not open file at {path:?}. Problem was: {e:?}"))?;

    let object = serde_yaml::from_reader(&file)
        .inspect_err(|e| 
            log::warn!("Deserialization of object in file {path:?} failed because of {e:?}"))?;

    Ok(object)
}

fn load_characters(character_folder: PathBuf) -> anyhow::Result<Vec<FileCharacter>> {
    load_object_vector(character_folder)
}

fn load_items(items_folder: PathBuf) -> anyhow::Result<Vec<ItemType>> {
    load_object_vector(items_folder)
}

pub struct CampaignFolder {
    pub meta: FileMeta,
    pub item_registry: ItemRegistry,
    pub characters: Vec<FileCharacter>,
    pub shops: Vec<FileShop>,
}

impl CampaignFolder {
    pub fn empty(name: String) -> Self {
        Self {
            meta: FileMeta { name: name, shops: vec![] },
            item_registry: ItemRegistry { items: vec![] },
            characters: vec![],
            shops: vec![],
        }
    }
}

fn load_campaign_folder(folder_path: PathBuf) -> anyhow::Result<CampaignFolder> {
    let items_path = folder_path.join("items"); // @todo make this some global const
    let items = load_items(items_path)
        .unwrap_or(vec![]);
    let item_registry = ItemRegistry { items };
    
    let character_path = folder_path.join("characters"); // @todo make this some global const
    let characters = load_characters(character_path) 
        .unwrap_or(vec![]);

    let f = std::fs::File::open(folder_path.join("simp.yaml"))?;
    let meta: FileMeta = serde_yaml::from_reader(f)?;
    
    let cf = CampaignFolder {
        characters,
        meta,
        item_registry,
        shops: vec![],
    };
    
    Ok(cf)

}



pub fn setup_logger() -> anyhow::Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] @ [{f}:{L}]: {m}{n}",
        )))
        .build("logs/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}

fn main() {
    match setup_logger() {
        Ok(()) => {},
        Err(e) => println!("Logger could not be initialized"),
    };

    log::info!("Hi there!");

    let args = CliArgs::parse();

    let result_work_campaign = if let Some(path) = args.campaign_file {
        load_campaign_folder(PathBuf::from(path))
    } else {
        Err(anyhow::format_err!("{}", "some error"))
    };

    // let result_work_campaign = load_campaign_folder(camp_file);

    let campaign_folder = match result_work_campaign {
        Ok(campaign) => campaign,
        Err(e) => {
            log::error!("Could not load campaign. {:?}", e);
            CampaignFolder::empty("Tina's Kampagne".into())
        }
    };

    let boxed = Box::new(campaign_folder);
    let persistent_folder: &'static mut CampaignFolder = Box::leak(boxed);

    let (mut campaign, item_registry) = (persistent_folder).into();

    let boxed = Box::new(campaign);
    let persistent_campaign: &'static mut Campaign = Box::leak(boxed);

    let inter = EngNerdI18n {};

    let s = ui::app::run_app(persistent_campaign, &inter);
}
