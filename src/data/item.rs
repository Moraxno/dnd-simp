use crate::registry::ItemType;

pub type ItemIdentifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    Unresolved(ItemIdentifier),
    Concrete(&'a ItemType)
}