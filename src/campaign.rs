use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::data::shop::Shop;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub name: String,
    pub shops: Vec<Shop>,
}

pub struct WorkCampaign {
    pub name: String,
    pub shops: Vec<Rc<RefCell<Shop>>>,
}

impl WorkCampaign {
    pub fn new(name: String) -> Self {
        Self {
            name,
            shops: vec![],
        }
    }
}

impl From<WorkCampaign> for Campaign {
    fn from(value: WorkCampaign) -> Self {
        Self {
            name: value.name,
            shops: value
                .shops
                .into_iter()
                .map(|rc| rc.borrow().to_owned())
                .collect(),
        }
    }
}

impl From<Campaign> for WorkCampaign {
    fn from(value: Campaign) -> Self {
        Self {
            name: value.name,
            shops: value
                .shops
                .into_iter()
                .map(|shop| Rc::new(RefCell::new(shop)))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Campaign;

    #[test]
    fn new_campaign_is_empty() {
        let e = WorkCampaign::new("New Campaign".into());
        assert_eq!(e.shops.len(), 0)
    }
}
