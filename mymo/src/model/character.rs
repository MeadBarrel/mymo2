use enum_map::EnumMap;
use super::attribute::Attribute;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Character {
    pub attributes: EnumMap<Attribute, u32>,
}
