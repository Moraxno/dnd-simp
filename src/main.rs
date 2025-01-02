use std::io::Read;

use campaign::{Campaign, WorkCampaign};
use clap::Parser;

mod apis;
mod campaign;
mod data;
mod meta;
mod registry;
mod ui;

mod state;


use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    campaign_file: Option<String>,
}

fn load_campaign_file(maybe_filepath: Option<String>) -> anyhow::Result<Campaign> {
    if let Some(filepath) = maybe_filepath {
        let mut f = std::fs::File::open(filepath)?;
        let mut buf = String::new();
        let _ = f.read_to_string(&mut buf)?;
        let content = buf.as_str();
        let campaign = serde_yaml::from_str(content)?;
        Ok(campaign)
    } else {
        Ok(campaign::WorkCampaign::new("Tina's und Sina's Kampagne".into()).into())
    }
}

pub fn setup_logger() -> anyhow::Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] @ [{f}:{L}]: {m}{n}")))
        .build("logs/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug))?;

    log4rs::init_config(config)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup_logger()?;

    log::info!("Hi there!");

    let args = CliArgs::parse();

    let campaign = load_campaign_file(args.campaign_file)?;

    let work_campaign: WorkCampaign = campaign.into();

    let boxed_campaign = Box::new(work_campaign);
    let campaign_ref: &'static mut WorkCampaign = Box::leak(boxed_campaign);

    // println!("{s:?}");

    // let f = std::fs::File::create("assets/outshop.yaml")?;
    // serde_yaml::to_writer(f, &s)?;
    // let d: String = serde_yaml::from_reader(f)?;
    // println!("Read YAML string: {}", d);

    let s = ui::app::run_app(campaign_ref);

    drop(s);

    Ok(())
}
