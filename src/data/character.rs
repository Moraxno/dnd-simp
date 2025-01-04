use serde::{Deserialize, Serialize};


use super::item::{Item, ItemIdentifier};

#[derive(Debug, Clone)]
pub struct Character<'a> {
    pub name: String,
    pub wish_list: Vec<Item<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCharacter {
    pub name: String,
    pub wish_list: Vec<ItemIdentifier>,
}

