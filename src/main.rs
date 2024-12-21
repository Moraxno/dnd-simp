use std::io::Read;

use campaign::Campaign;
use clap::Parser;

mod apis;
mod campaign;
mod meta;
mod registry;
mod shop;
mod ui;

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
        Ok(campaign::Campaign::new("Tina's und Sina's Kampagne".into()))
    }
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let mut s = load_campaign_file(args.campaign_file)?;

    println!("{s:?}");

    // let f = std::fs::File::create("assets/outshop.yaml")?;
    // serde_yaml::to_writer(f, &s)?;
    // let d: String = serde_yaml::from_reader(f)?;
    // println!("Read YAML string: {}", d);

    ui::app::run_app(&mut s);

    Ok(())
}
