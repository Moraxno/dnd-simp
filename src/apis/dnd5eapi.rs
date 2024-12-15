use std::fmt::Write as _;
use std::{fmt};

use regex::Regex;
use serde::{Deserialize, Serialize};

use anyhow::anyhow;

use mockall::*;
use mockall::predicate::*;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Dnd5eApiItemRarity {
    name: String
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

const DND5EAPI_BASEURL: &str = "https://www.dnd5eapi.co";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Dnd5eApiMagicItemList {
    count: u64,
    results: Vec<Dnd5eApiMagicItemListing>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Dnd5eApiMagicItemListing {
    index: String,
    name: String,
    url: String
}

#[derive(Debug)]
pub enum Dnd5eApiError {
    ItemNotFound,
}

#[automock]
trait PerformsRequest {
    fn request_from_sub_url(&self, sub_url: &str) -> anyhow::Result<String>;
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

    pub fn download_item(&self, sub_url: &str, requester: &dyn PerformsRequest) -> anyhow::Result<Dnd5eApiItem> {
        let resp = requester.request_from_sub_url(sub_url)?;
        let item = Dnd5eApiItem::from_json(&resp)?;

        Ok(item)
    }

    pub fn search_item(&self, item_regex: &str, requester: &dyn PerformsRequest) -> anyhow::Result<Dnd5eApiItem> {
        let sub_url = self.search_for_url(item_regex)?;
        self.download_item(&sub_url, requester)
    }
}


pub struct Dnd5eApiRequester {

}

impl PerformsRequest for Dnd5eApiRequester {
    fn request_from_sub_url(&self, sub_url: &str) -> anyhow::Result<String> {
        let mut url = String::new();
        write!(url, "{DND5EAPI_BASEURL}/{sub_url}")?;

        Ok(reqwest::blocking::get(url)?.text()?)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use super::{Dnd5eApiItem, Dnd5eApiMagicItemList, MockPerformsRequest};

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

    #[test]
    fn searching_and_download_works() -> anyhow::Result<()> {
        let list_contents = std::fs::read_to_string("assets/dnd5eapi-itemlist.json")?;
        let l = Dnd5eApiMagicItemList::from_json(&list_contents)?;

        let item_contents = std::fs::read_to_string("assets/dnd5eapi/magic-items/apparatus-of-the-crab.json")?;
        let expected_item = Dnd5eApiItem::from_json(item_contents.as_str())?;

        let mut requester = MockPerformsRequest::new();
        requester
            .expect_request_from_sub_url()
            .with(eq("/api/magic-items/apparatus-of-the-crab"))
            .times(1)
            .returning(move |_| Ok(item_contents.clone()));

        let constructed_item = l.search_item(".*crab.*", &requester)?;
        assert_eq!(constructed_item, expected_item);

        Ok(())
    }
}