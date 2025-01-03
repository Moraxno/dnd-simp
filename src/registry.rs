use std::fmt::Display;

use ratatui::{
    style::{Style, Stylize},
    text::Span,
};
use serde::{Deserialize, Serialize};

use crate::{data::item::ItemIdentifier, ui::display::AsRatatuiSpan};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
    Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategory {
    WondrousItem,
    SimpleWeapon,
    Wand,
}

impl Display for ItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ItemCategory::WondrousItem => "Wondrous Item",
            ItemCategory::SimpleWeapon => "Simple Weapon",
            ItemCategory::Wand => "Wand",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemType {
    pub identifier: ItemIdentifier,
    pub name: String,
    pub details: String,
    pub rarity: Rarity,
    pub category: ItemCategory,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rarity::Common => "Common".into(),
            Rarity::Uncommon => "Uncommon".into(),
            Rarity::Rare => "Rare".into(),
            Rarity::VeryRare => "Very Rare".into(),
            Rarity::Legendary => "Legendary".into(),
            Rarity::Artifact => "Artifact".into(),
        };

        f.write_str(s)
    }
}

impl Rarity {
    pub fn as_symbol(&self) -> String {
        match self {
            Rarity::Common => "C".into(),
            Rarity::Uncommon => "U".into(),
            Rarity::Artifact => "A".into(),
            Rarity::Rare => "R".into(),
            Rarity::VeryRare => "V".into(),
            Rarity::Legendary => "L".into(),
            _ => "I am to lazy".into(),
        }
    }
}

impl AsRatatuiSpan for Rarity {
    fn as_span(&self) -> Span {
        let base_span = Span::raw(self.to_string());
        match self {
            Rarity::Common => base_span.style(Style::default().gray().italic()),
            Rarity::Uncommon => base_span.style(Style::default().white().italic()),
            Rarity::Rare => base_span.style(Style::default().green().italic()),
            Rarity::VeryRare => base_span.style(Style::default().magenta().italic()),
            Rarity::Legendary => base_span.style(Style::default().red().italic()),
            Rarity::Artifact => base_span.style(Style::default().red().underlined().italic()),
        }
    }
}

pub type CostExpressionFunction = dyn Fn(&ItemType) -> String;

pub fn xanathar_magic_item_cost(item: &ItemType) -> String {
    match item.rarity {
        Rarity::Common => "(1d6 + 1) * 10".into(),
        Rarity::Uncommon => "1d6 * 100".into(),
        Rarity::Rare => "2d10 * 1000".into(),
        Rarity::VeryRare => "(1d4 + 1) * 10000".into(),
        Rarity::Legendary => "2d6 * 25000".into(),
        Rarity::Artifact => "2d6 * 25000".into(),
    }
}

impl ItemType {
    pub fn new(name: String, rarity: Rarity, category: ItemCategory, details: String) -> Self {
        Self {
            identifier: name.clone(),
            name,
            rarity,
            category,
            details,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ItemRegistry {
    item_types: Vec<ItemType>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self { item_types: vec![] }
    }

    pub fn add(&mut self, item_type: ItemType) {
        self.item_types.push(item_type);
    }

    pub fn items(&self) -> &[ItemType] {
        self.item_types.as_slice()
    }

    pub fn to_yaml(&mut self) -> anyhow::Result<String> {
        Ok(serde_yaml::to_string(&self)?)
    }

    pub fn from_yaml(yaml_text: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(yaml_text)?)
    }

    pub fn from_reader<R: std::io::Read>(yaml_reader: R) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(yaml_reader)?)
    }
}

pub enum ItemQuantity {
    Stocked(u64),
    Infinite,
}
