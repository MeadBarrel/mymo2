use serde::{Serialize, Deserialize};
use enum_map::Enum;
use strum::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, EnumIter)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Psyche,
}

impl Attribute {
    pub fn name(&self) -> String {
        use Attribute::*;        
        match self {
            Strength => "Strength",
            Dexterity => "Dexterity",
            Constitution => "Constitution",
            Intelligence => "Intelligence",
            Psyche => "Psyche",
        }.to_string()
    }

    pub fn default_cap(&self) -> u32 {
        use Attribute::*;
        match self {
            Strength => 97,
            Dexterity => 100,
            Constitution => 95,
            Intelligence => 99,
            Psyche => 89
        }
    }
}
