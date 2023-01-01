use std::{vec, collections::VecDeque, default};
use serde::{Serialize, Deserialize};
use enum_map::{Enum, EnumMap};
use strum::EnumIter;

use super::attribute::Attribute;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Serialize, Deserialize, Default)] 
pub enum Sex {
    #[default]
    Male,
    Female
}

#[derive(Debug, Clone, Copy, EnumIter, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Clade {
    #[default]
    Human,
    Alvarin,
    Oghmir,
    Thursar
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Race {
    // Human
    #[default]
    Tindremen,
    Sidoian,
    Sarducaan,
    Khurite,
    Kallard,

    // Alvarin
    Veela,
    Sheevra,

    // Oghmir
    Blainn,
    Huergar,

    // Thursar
    Thursar,
}

#[derive(Debug, Clone, Copy, EnumIter, Enum, Serialize, Deserialize)]
pub enum Parent {
    First,
    Second,
    Third,
    Fourth,
}

impl Sex {
    pub fn name(self) -> String {
        match self {
            Sex::Male => "Male",
            Sex::Female => "Female"
        }.to_string()
    }
}

impl Clade {
    pub fn name(self) -> String {
        use Clade::*;
        match self {
            Human => "Human",
            Alvarin => "Alvarin",
            Oghmir => "Oghmir",
            Thursar => "Thursar",
        }.to_string()
    }

    pub fn default_parents(self) -> EnumMap<Parent, Race> {
        use Race::*;
        use Parent::*;

        match self {
            Clade::Human => vec![
                (First, Tindremen), (Second, Tindremen), (Third, Tindremen), (Fourth, Tindremen)]
                .into_iter().collect(),
            Clade::Alvarin => vec![(First, Veela), (Second, Veela), (Third, Veela), (Fourth, Veela)]
                .into_iter().collect(),
            Clade::Oghmir => vec![
                (First, Blainn), (Second, Blainn), (Third, Blainn), (Fourth, Blainn)]
                .into_iter().collect(),
            Clade::Thursar => vec![
                (First, Thursar), (Second, Thursar), (Third, Tindremen), (Fourth, Tindremen)]
                .into_iter().collect()
        }
    }
}

impl Race {

    /// Return attribute cap modifier for this race, assuming Tindremen to be
    /// the default
    pub fn attribute_cap(&self, attribute: Attribute) -> i8 {
        use Race::*;
        use Attribute::*;

        match self {
            Tindremen => 0,
            Sidoian => match attribute {
                Strength => 2,
                Dexterity => -1,
                Constitution => 2,
                Intelligence => 1,
                Psyche => -3,
            }
            Sarducaan => match attribute {
                Strength => -2,
                Dexterity => 0,
                Constitution => 2,
                Intelligence => -1,
                Psyche => 3
            },
            Khurite => match attribute {
                Strength => 1,
                Dexterity => 2,
                Constitution =>  4,
                Intelligence =>  -5,
                Psyche => 1,
            },
            Kallard => match attribute {
                Strength => 2,
                Dexterity => -1,
                Constitution =>  5,
                Intelligence =>  -7,
                Psyche => 1
            },            

            Veela => match attribute {
                Strength => -3,
                Dexterity => 5,
                Constitution =>  1,
                Intelligence =>  -2,
                Psyche => 3,
            },
            Sheevra => match attribute {
                Strength => -4,
                Dexterity => 4,
                Constitution =>  1,
                Intelligence =>  0,
                Psyche => 4,                
            },

            Blainn => match attribute {
                Strength => 4,
                Dexterity => -2,
                Constitution =>  9,
                Intelligence =>  1,
                Psyche => -1,
            },
            Huergar => match attribute {
                Strength => 4,
                Dexterity => -2,
                Constitution =>  7,
                Intelligence =>  2,
                Psyche => 0
            },

            Thursar => match attribute {
                Strength => 5,
                Dexterity => -1,
                Constitution =>  6,
                Intelligence =>  -6,
                Psyche => -10,
            },            
        }
    }

    /// Return minimum height modifier for this race, assuming tindremene to be
    /// the default
    pub fn min_height(&self) -> i32 {
        use Race::*;
        match self {
            Tindremen => 0,
            Sidoian => 0,
            Sarducaan => -1,
            Khurite => -1,
            Kallard => 2,

            Veela => -2,
            Sheevra => -3,

            Blainn => -4,
            Huergar => -5,

            Thursar => 3,
        }
    }

    /// Return maximum height modifier for this race, assuming tindremene to be
    /// the default
    pub fn max_height(&self) -> i32 {
        use Race::*;
        match self {
            Tindremen => 0,
            Sidoian => -1,
            Sarducaan => -4,
            Khurite => -5,
            Kallard => 2,

            Veela => -5,
            Sheevra => -6,

            Blainn => -7,
            Huergar => -8,

            Thursar => 4,
        }
    }

    pub fn attribute_pool(&self) -> i8 {
        use Race::*;
        match self {
            Tindremen => 0,
            Sidoian => -5,
            Sarducaan => 0,
            Khurite => -11,
            Kallard => -9,

            Veela => -6,
            Sheevra => -4,

            Blainn => -13,
            Huergar => -13,

            Thursar => -14,
        }
    }
}


/// Return available parent races for this clade at parent index
pub fn avail_parents(clade: Clade, parent: Parent) -> Vec<Race> {
    use Race::*;
    match clade {
        Clade::Human => vec! [
            Tindremen,
            Sidoian,
            Sarducaan,
            Khurite,
            Kallard,
        ],
        Clade::Alvarin => vec! [
            Veela,
            Sheevra,
        ],
        Clade::Oghmir => vec! [
            Blainn,
            Huergar,
        ],
        Clade::Thursar => match parent {
            Parent::First | Parent::Second => vec![Thursar],
            Parent::Third | Parent::Fourth => vec![
                Tindremen,
                Sidoian,
                Sarducaan,
                Khurite,
                Kallard,
            ]
        }
    }
}