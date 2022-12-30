use serde::{Serialize, Deserialize};
use enum_map::Enum;

#[derive(Debug, Clone, Serialize, Deserialize, Enum)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Psyche,
}