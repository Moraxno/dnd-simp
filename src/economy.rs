use crate::shop::{self, Shop};

pub struct Economy {
    shops: Vec<Shop>,
}

impl Economy {
    pub fn new() -> Self {
        Self { shops: vec![] }
    }

    pub fn get_shops(&self) -> &[Shop] {
        self.shops.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::Economy;

    #[test]
    fn new_economy_is_empty() {
        let e = Economy::new();
        assert_eq!(e.get_shops().len(), 0)
    }
}
