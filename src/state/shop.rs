use crate::data::{item::ItemType, shop::{Shop, StockedItem}};

pub struct ShopState<'a> {
    shop: &'a Shop<'a>,

    offers: Vec<&'a StockedItem<'a>>,
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

    pub fn get_offers(&self) -> &[&StockedItem] {
        self.offers.as_slice()
    }

    pub fn regenerate_offers(&mut self) -> &[&StockedItem<'a>] {
        self.offers = self.shop.produce_offer(self.num_offers);
        self.offers.as_slice()
    }
}
