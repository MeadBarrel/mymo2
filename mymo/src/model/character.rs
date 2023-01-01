use enum_map::EnumMap;
use super::{attribute::Attribute, Race, Parent, Clade};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Character {
    pub clade: Clade,
    pub parents: EnumMap<Parent, Race>,
    pub attributes: EnumMap<Attribute, u32>,
}

impl Character {
    /// Get attribute cap modifier for this character at age 18, height 150cm, 
    /// 100% weight, assuming full tindremene to be the default.
    pub fn attribute_cap(&self, attribute: Attribute) -> u32 {
        let result = attribute.default_cap() as i32 
            + self.parents.values().map(|x| x.attribute_cap(attribute)).sum::<i8>() as i32;
        result.max(0) as u32
    }
}