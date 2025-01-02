use ratatui::{
    style::{Style, Stylize},
    text::Span,
};
use serde::{Deserialize, Serialize};

use crate::ui::display::{AsRatatuiSpan, HasCostExpression};

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
pub struct ItemType {
    pub name: String,
    pub details: String,
    pub rarity: Rarity,
}

impl Rarity {
    pub fn as_string(&self) -> String {
        match self {
            Rarity::Common => "Common".into(),
            Rarity::Common => "Uncommon".into(),
            Rarity::Rare => "Rare".into(),
            Rarity::VeryRare => "Very Rare".into(),
            Rarity::Legendary => "Legendary".into(),
            Rarity::Artifact => "Artifact".into(),
            _ => "I am to lazy".into(),
        }
    }

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
        let base_span = Span::raw(self.as_string());
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

impl HasCostExpression for ItemType {
    fn price_expr(&self) -> String {
        match self.rarity	 {
            Rarity::Common => "(1d6 + 1) * 10".into(),
            Rarity::Uncommon => "1d6 * 100".into(),
            Rarity::Rare => "2d10 * 1000".into(),
            Rarity::VeryRare => "(1d4 + 1) * 10000".into(),
            Rarity::Legendary => "2d6 * 25000".into(),
            Rarity::Artifact => "2d6 * 25000".into(),
        }
    }
}

impl ItemType {
    pub fn new(name: String, rarity: Rarity, details: String) -> Self {
        Self {
            name,
            rarity,
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
