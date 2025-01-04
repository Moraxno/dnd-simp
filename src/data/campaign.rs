use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::registry::ItemRegistry;

use super::{character::FileCharacter, item::ItemType, shop::FileShop};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    pub name: String,
}

fn load_object_vector<T: for<'a> Deserialize<'a>>(folder: PathBuf) -> anyhow::Result<Vec<T>> {
    let entries = std::fs::read_dir(folder)?;

    let objects = entries
        .filter_map(|maybe_entry| 
            match maybe_entry {
                Ok(entry) => load_object(&entry.path()).ok(),
                Err(_) => None
            })
        .collect();

    Ok(objects)
}

fn load_object<T: for<'a> Deserialize<'a>>(path: &PathBuf) -> anyhow::Result<T>{
    let file = std::fs::File::open(path)
        .inspect_err(|e| 
            log::warn!("Could not open file at {path:?}. Problem was: {e:?}"))?;

    let object = serde_yaml::from_reader(&file)
        .inspect_err(|e| 
            log::warn!("Deserialization of object in file {path:?} failed because of {e:?}"))?;

    Ok(object)
}

fn load_characters(character_folder: PathBuf) -> anyhow::Result<Vec<FileCharacter>> {
    load_object_vector(character_folder)
}

fn load_items(items_folder: PathBuf) -> anyhow::Result<Vec<ItemType>> {
    load_object_vector(items_folder)
}

pub struct CampaignFolder {
    pub meta: FileMeta,
    pub item_registry: ItemRegistry,
    pub characters: Vec<FileCharacter>,
    pub shops: Vec<FileShop>,
}

impl CampaignFolder {
    pub fn empty(name: String) -> Self {
        Self {
            meta: FileMeta { name: name },
            item_registry: ItemRegistry { items: vec![] },
            characters: vec![],
            shops: vec![],
        }
    }
}

fn load_campaign_folder(folder_path: PathBuf) -> anyhow::Result<CampaignFolder> {
    let items_path = folder_path.join("items"); // @todo make this some global const
    let items = load_items(items_path)
        .unwrap_or(vec![]);
    let item_registry = ItemRegistry { items };
    
    let character_path = folder_path.join("characters"); // @todo make this some global const
    let characters = load_characters(character_path) 
        .unwrap_or(vec![]);

    let f = std::fs::File::open(folder_path.join("simp.yaml"))?;
    let meta: FileMeta = serde_yaml::from_reader(f)?;
    
    let cf = CampaignFolder {
        characters,
        meta,
        item_registry,
        shops: vec![],
    };
    
    Ok(cf)

}

