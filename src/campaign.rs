use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::data::{campaign::CampaignFolder, character::{Character, FileCharacter}, item::{Item, ItemType}, shop::Shop};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FileStorageVersion {
    V1,
}



pub struct Campaign<'a> {
    pub name: String,
    pub characters: Vec<Character<'a>>,
    pub shops: Vec<Rc<RefCell<Shop<'a>>>>,
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
    use crate::CampaignFolder;

    #[test]
    fn new_campaign_is_empty() {
        let e = CampaignFolder::empty("New Campaign".into());
        assert_eq!(e.shops.len(), 0)
    }
}
