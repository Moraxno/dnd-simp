use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::data::{character::Character, shop::Shop};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FileStorageVersion {
    V1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCampaign {
    pub name: String,
    pub shops: Vec<Shop>,


}

pub struct WorkCampaign {
    pub name: String,
    pub characters: Vec<Character>,
    pub shops: Vec<Rc<RefCell<Shop>>>,
}

impl WorkCampaign {
    pub fn new(name: String) -> Self {
        Self {
            name,
            characters: vec![],
            shops: vec![],
        }
    }
}

impl From<WorkCampaign> for FileCampaign {
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

impl From<FileCampaign> for WorkCampaign {
    fn from(value: FileCampaign) -> Self {
        Self {
            name: value.name,
            characters: vec![],
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
    use crate::WorkCampaign;

    #[test]
    fn new_campaign_is_empty() {
        let e = WorkCampaign::new("New Campaign".into());
        assert_eq!(e.shops.len(), 0)
    }
}
