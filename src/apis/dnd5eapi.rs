use std::{fmt};

use regex::Regex;
use serde::{Deserialize, Serialize};

use anyhow::anyhow;

#[derive(Debug, Deserialize, Serialize)]
struct Dnd5eApiItem {
    index: String,
    name: String,
    equipment_category: Dnd5eApiEquipmentCategory,
    rarity: Dnd5eApiItemRarity,
    variants: Vec<()>,
    variant: bool,
    desc: Vec<String>,
    url: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Dnd5eApiItemRarity {
    name: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Dnd5eApiEquipmentCategory {
    index: String,
    name: String,
    url: String,
}

impl Dnd5eApiItem {
    pub fn from_json(json_text: &str) -> anyhow::Result<Self> {
        let item: Self = serde_json::from_str(&json_text)?;
        Ok(item)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Dnd5eApiMagicItemList {
    count: u64,
    results: Vec<Dnd5eApiMagicItemListing>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dnd5eApiMagicItemListing {
    index: String,
    name: String,
    url: String
}

#[derive(Debug)]
pub enum Dnd5eApiError {
    ItemNotFound,
}

impl fmt::Display for Dnd5eApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dnd5eApiError::ItemNotFound => write!(f, "item could not be found via search term"),
        }
    }
}

impl Dnd5eApiMagicItemList {
    pub fn from_json(json_text: &str) -> anyhow::Result<Self> {
        let list: Self = serde_json::from_str(json_text)?;
        Ok(list)
    }

    pub fn search_for_url(&self, item_regex: &str) -> anyhow::Result<String> {
        let re = Regex::new(item_regex)?;

        for item in &self.results {
            if re.find(item.name.as_str()).is_some() || re.find(item.index.as_str()).is_some() {
                return Ok(item.url.clone());
            }
        }

        Err(anyhow!(Dnd5eApiError::ItemNotFound))
    }
}

#[cfg(test)]
mod tests {
    use super::{Dnd5eApiItem, Dnd5eApiMagicItemList};

    #[test]
    fn item_is_parsed() -> anyhow::Result<()> {
        let contents = std::fs::read_to_string("assets/dnd5eapi-item.json")?;
        let i = Dnd5eApiItem::from_json(&contents)?;

        assert_eq!(i.name, "Cape of the Mountebank");
        Ok(())
    }

    #[test]
    fn list_is_parsed() -> anyhow::Result<()> {
        let contents = std::fs::read_to_string("assets/dnd5eapi-itemlist.json")?;
        let l = Dnd5eApiMagicItemList::from_json(&contents)?;

        assert_eq!(l.count, 362);
        assert_eq!(l.results[0].name, "Adamantine Armor");
        Ok(())
    }

    #[test]
    fn list_search_finds_item() -> anyhow::Result<()> {
        let contents = std::fs::read_to_string("assets/dnd5eapi-itemlist.json")?;
        let l = Dnd5eApiMagicItemList::from_json(&contents)?;
        let url = l.search_for_url(".*crab.*")?;

        assert_eq!(url, "/api/magic-items/apparatus-of-the-crab");
        Ok(())
    }

    #[test]
    fn list_search_fails_on_bad_search_term() -> anyhow::Result<()> {
        let contents = std::fs::read_to_string("assets/dnd5eapi-itemlist.json")?;
        let l = Dnd5eApiMagicItemList::from_json(&contents)?;
        let maybe_error = l.search_for_url("Though this be madness, yet there is method in't.");

        assert!(maybe_error.is_err());
        Ok(())
    }
}