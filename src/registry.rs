use std::{fmt::Display, ops::Index};

use ratatui::{
    style::{Style, Stylize},
    text::Span,
};
use serde::{Deserialize, Serialize};

use crate::{data::item::{ItemCategory, ItemIdentifier, ItemType, Rarity}, ui::display::AsRatatuiSpan};

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



#[derive(Debug, Deserialize, Serialize)]
pub struct ItemRegistry {
    pub items: Vec<ItemType>
}

impl ItemRegistry {
    pub fn link_character<'a>(&'a self, character: FileCharacter) -> Character<'a> {
        Character {
            name: character.name,
            wish_list: link_wishlist(&self.items, character.wish_list)
        }
    }
}

fn link_wishlist(items: &Vec<ItemType>, wish_list: Vec<String>) -> Vec<Item<'_>> {
    wish_list
        .into_iter()
        .map(|item_identifier| 
            items
                .iter()
                .find(|item| 
                    item.identifier == item_identifier)
                .map_or_else(
                    || Item::Unresolved(item_identifier), 
                    |item| Item::Concrete(item)))
        .collect()
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

    pub fn get(&self, key: ItemIdentifier) -> Option<&ItemType> {
        for i in self.item_types.iter() {
            if i.identifier == key {
                return Some(i);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: ItemIdentifier) -> Option<&mut ItemType> {
        for i in self.item_types.iter_mut() {
            if i.identifier == key {
                return Some(i);
            }
        }

        None
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