use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Rare,
    VeryRare,
    Legendary,
    Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemType {
    pub name: String,
    pub rarity: Rarity,
}

impl Rarity {
    pub fn as_string(&self) -> String {
        match self {
            Rarity::Common => "common".into(),
            Rarity::Artifact => "ARTIFACT".into(),
            Rarity::Rare => "rare".into(),
            _ => "I am to lazy".into(),
        }
    }

    pub fn as_symbol(&self) -> String {
        match self {
            Rarity::Common => "C".into(),
            Rarity::Artifact => "A".into(),
            Rarity::Rare => "R".into(),
            Rarity::VeryRare => "V".into(),
            Rarity::Legendary => "L".into(),
            _ => "I am to lazy".into(),
        }
    }
}

impl ItemType {
    pub fn new(name: String, rarity: Rarity) -> Self {
        Self { name, rarity }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ItemRegistry {
    item_types: Vec<ItemType>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self { item_types: vec![] }
    }

    pub fn add(&mut self, item_type: ItemType) {
        self.item_types.push(item_type);
    }

    pub fn items(&self) -> &[ItemType] {
        self.item_types.as_slice()
    }

    pub fn to_yaml(&mut self) -> anyhow::Result<String> {
        Ok(serde_yaml::to_string(&self)?)
    }

    pub fn from_yaml(yaml_text: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(yaml_text)?)
    }

    pub fn from_reader<R: std::io::Read>(yaml_reader: R) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(yaml_reader)?)
    }
}
