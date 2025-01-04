use std::fmt::Display;
use serde::{Deserialize, Serialize};

pub type ItemIdentifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    Unresolved(ItemIdentifier),
    Concrete(&'a ItemType)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemType {
    pub identifier: ItemIdentifier,
    pub name: String,
    pub details: String,
    pub rarity: Rarity,
    pub category: ItemCategory,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
    Artifact,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

