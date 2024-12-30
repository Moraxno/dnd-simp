use crate::{data::shop::Shop, registry::ItemType};

pub struct ShopState<'a> {
    shop: &'a Shop,

    offers: Vec<&'a ItemType>,
    num_offers: u8,
}

impl<'a> ShopState<'a> {
    pub fn new(shop: &'a Shop) -> ShopState<'a> {
        Self {
            shop,
            offers: vec![],
            num_offers: 3,
        }
    }

    pub fn get_offers(&self) -> &[&ItemType] {
        self.offers.as_slice()
    }

    pub fn regenerate_offers(&mut self) -> &[&ItemType] {
        self.offers = self.shop.produce_offer(self.num_offers);
        self.offers.as_slice()
    }
}
