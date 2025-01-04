use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{data::{campaign::{CampaignFolder, FileMeta}, character::{Character, FileCharacter}, item::{Item, ItemType}, shop::Shop}, registry::ItemRegistry};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FileStorageVersion {
    V1,
}



pub struct Campaign<'a> {
    pub name: String,
    pub characters: Vec<Character<'a>>,
    pub shops: Vec<Rc<RefCell<Shop<'a>>>>,
}

impl CampaignFolder {
    fn destructure<'a>(&'a mut self) -> (Campaign<'a>, &'a mut ItemRegistry) {
        let campaign = Campaign::from_files(self.meta.clone(), self.characters.clone(), &self.item_registry);

        (campaign, &mut self.item_registry)
    }
}
rust 
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
