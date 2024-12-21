use serde::{Deserialize, Serialize};

use crate::shop::Shop;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub name: String,
    shops: Vec<Shop>,
}

impl Campaign {
    pub fn new(name: String) -> Self {
        Self { name, shops: vec![] }
    }

    pub fn get_shops(&self) -> &[Shop] {
        self.shops.as_slice()
    }

    pub fn add_shop(&mut self, shop: Shop) {
        self.shops.push(shop);
    }
}

#[cfg(test)]
mod tests {
    use super::Shop;

    #[test]
    fn new_campaign_is_empty() {
        let e = Shop::new();
        assert_eq!(e.get_shops().len(), 0)
    }
}