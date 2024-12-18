use std::io::Write;

use registry::ItemType;


mod economy;
mod shop;
mod apis;
mod registry;
mod meta;
mod ui;

fn main() -> anyhow::Result<()> {
    let s = shop::Shop::new("Tina's und Sina's".into());
    println!("Hello, world!");

    // let f = std::fs::File::create("assets/outshop.yaml")?;
    // serde_yaml::to_writer(f, &s)?;
    // let d: String = serde_yaml::from_reader(f)?;
    // println!("Read YAML string: {}", d);


    println!("{s:?}");

    ui::app::run_app();

    Ok(())
}
