use serde::{Deserialize, Serialize};


use crate::registry::ItemRegistry;

use super::item::{Item, ItemIdentifier};

#[derive(Debug, Clone)]
pub struct Character<'a> {
    pub state: FileCharacter,
    pub registry: &'a ItemRegistry
}

impl<'a> Character<'a> {
    pub fn name(&self) -> String {
        self.state.name
    }

    // maybe cache this?
    pub fn wish_list(&self) -> Vec<Item<'a>> {
        self.state.wish_list
            .iter()
            .map(|id| self.registry
                .get(id)
                .map_or_else(|| Item::Unresolved(id.clone()), |item| Item::Concrete(item)))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCharacter {
    pub name: String,
    pub wish_list: Vec<ItemIdentifier>,
}

