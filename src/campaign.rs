use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{data::{character::{Character, FileCharacter}, item::Item, shop::Shop}, registry::ItemType, CampaignFolder};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FileStorageVersion {
    V1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    pub name: String,
    pub shops: Vec<Shop>,
}

pub struct Campaign<'a> {
    pub name: String,
    pub characters: Vec<Character<'a>>,
    pub shops: Vec<Rc<RefCell<Shop>>>,
}

impl<'a> From<&'a CampaignFolder> for (Campaign<'a>, &'a ItemRegistry) {
    fn from(value: &'a CampaignFolder) -> (Campaign<'a>, &'a ItemRegistry) {
        let campaign = Campaign::from_files(value.meta.clone(), value.characters.clone(), &value.item_registry);

        (campaign, &value.item_registry)
    }
}

// @todo i hate this, we have to change this
impl<'a> From<&'a mut CampaignFolder> for (Campaign<'a>, &'a ItemRegistry) {
    fn from(value: &'a mut CampaignFolder) -> (Campaign<'a>, &'a ItemRegistry) {
        let campaign = Campaign::from_files(value.meta.clone(), value.characters.clone(), &value.item_registry);

        (campaign, &value.item_registry)
    }
}

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

impl<'a> Campaign<'a> {
    pub fn from_files(meta: FileMeta, characters: Vec<FileCharacter>, registry: &'a ItemRegistry) -> Self {
        Self {
            name: meta.name,
            characters: characters
                .into_iter()
                .map(|ch| 
                    registry.link_character(ch))
                .collect(),
            shops: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::WorkCampaign;

    #[test]
    fn new_campaign_is_empty() {
        let e = Campaign::new("New Campaign".into());
        assert_eq!(e.shops.len(), 0)
    }
}
