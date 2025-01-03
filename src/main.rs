use std::io::Read;
use std::path::{Path, PathBuf};

use campaign::{FileCampaign};
use clap::Parser;

mod apis;
mod campaign;
mod data;
mod meta;
mod registry;
mod ui;

mod state;

use data::character::{Character, FileCharacter};
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
    let mut objects = vec![];

    let entries = std::fs::read_dir(folder)?;
    for entry in entries {
        match entry {
            Ok(entry) => {
                log::info!("Looking into {:?}", entry);
                match std::fs::File::open(entry.path()) {
                    Ok(file) => {
                        match serde_yaml::from_reader(&file) {
                            Ok(obj) => objects.push(obj),
                            Err(e) => log::warn!("Could not read in file {:?} because of {:?}", file, e)
                        }
                    },
                    Err(_) => { /* @todo properly signify this, ignore these for now... */ }
                }
            },
            Err(_) => { /* @todo properly signify this, ignore these for now... */ }
        }
    }

    Ok(objects)
}

fn load_characters(character_folder: PathBuf) -> anyhow::Result<Vec<FileCharacter>> {
    load_object_vector(character_folder)
}

fn load_items(items_folder: PathBuf) -> anyhow::Result<Vec<ItemType>> {
    load_object_vector(items_folder)
}

fn load_campaign_folder(maybe_folder: Option<PathBuf>) -> anyhow::Result<FileCampaign> {
    if let Some(filepath) = maybe_folder {
        let items_path = filepath.join("items"); // @todo make this some global const
        let items = load_items(items_path)
            .unwrap_or(vec![]);
        
        let character_path = filepath.join("characters"); // @todo make this some global const
        let characters = load_characters(character_path) 
            .unwrap_or(vec![]);


        let f = std::fs::File::open(filepath.join("camp.yaml"))?;
        let campaign: FileCampaign = serde_yaml::from_reader(f)?;
        let mut work_campaign: FileCampaign = campaign.into();
        work_campaign.characters = characters; // @todo: ugly, should be a constructor
        Ok(work_campaign)
    } else {
        Ok(campaign::FileCampaign::new("Tina's und Sina's Kampagne".into()))
    }
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

    let camp_file = if let Some(path) = args.campaign_file {
        Some(PathBuf::from(path))
    } else {
        None
    };

    let result_work_campaign = load_campaign_folder(camp_file);

    let work_campaign = match result_work_campaign {
        Ok(campaign) => campaign,
        Err(e) => {
            log::error!("Could not load campaign. {:?}", e);
            FileCampaign::new("New Campaign".into())
        }
    };

    let boxed_campaign = Box::new(work_campaign);
    let campaign_ref: &'static mut FileCampaign = Box::leak(boxed_campaign);

    let inter = EngNerdI18n {};

    let s = ui::app::run_app(campaign_ref, &inter);
}
