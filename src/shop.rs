use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

use crate::registry::ItemType;



#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    name: String,
    inventory: Vec<ItemType>,
}

impl Shop {
    pub fn new(name: String) -> Self {
        Self {
            name,
            inventory: vec![],
        }
    }

    pub fn get_inventory(&self) -> &[ItemType] {
        self.inventory.as_slice()
    }

    pub fn produce_offer(&self) -> Vec<&ItemType> {
        self.inventory
            .choose_multiple(&mut rand::thread_rng(), 3)
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
    fn shop_offer_is_less_or_equal_to_three() {
        let s = Shop::new("Tina's".to_string());
        assert!(s.produce_offer().len() <= 3)
    }
}