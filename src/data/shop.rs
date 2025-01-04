use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::num::NonZero;

use super::item::{ItemIdentifier, ItemType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stock {
    Stocked(NonZero<u32>),
    Sold,
    Infinite
}

#[derive(Debug, Clone)]
pub struct StockedItem<'a> {
    pub item_type: &'a ItemType,
    pub stock: Stock
}

#[derive(Debug, Clone)]
pub struct Shop<'a> {
    pub name: String,
    short_name: Option<String>,
    inventory: Vec<StockedItem<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStockedItem {
    pub identifier: ItemIdentifier,
    pub stock: Stock
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileShop {
    pub name: String,
    pub short_name: Option<String>,
    pub inventory: Vec<FileStockedItem>,
}

impl<'a> Shop<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            short_name: None,
            inventory: vec![],
        }
    }

    pub fn new_with_shorthand(name: String, short_name: String) -> Self {
        Self {
            name,
            short_name: Some(short_name),
            inventory: vec![],
        }
    }

    pub fn short_name(&self) -> String {
        if let Some(name) = &self.short_name {
            name.clone()
        } else {
            self.name[..20].into()
        }
    }

    pub fn get_inventory(&self) -> &[StockedItem] {
        self.inventory.as_slice()
    }

    pub fn produce_offer(&self, amount: u8) -> Vec<&StockedItem> {
        self.inventory
            .choose_multiple(&mut rand::thread_rng(), amount.into())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Shop;

    #[test]
    fn new_shop_is_empty() {
        let s = Shop::new("Tina's".to_string());
        assert_eq!(s.get_inventory().len(), 0)
    }

    #[test]
    fn shop_offer_is_less_or_equal_to_demand() {
        let s = Shop::new("Tina's".to_string());
        assert!(s.produce_offer(3).len() <= 3)
    }
}
