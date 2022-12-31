use std::vec;
use serde::{Serialize, Deserialize};
use enum_map::Enum;
use strum::EnumIter;

use super::attribute::Attribute;

#[derive(Debug, Clone, Copy, EnumIter, Default, Serialize, Deserialize)]
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

impl Race {
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

    // pub fn attributes(&self) -> EnumMap<Attribute, i8> {
    //     use Race::*;
    //     use Attribute::*;

    //     match self {
    //         Tindremen => EnumMap::default(),
    //         Sidoian => vec![
    //             (Strength, 2),
    //             (Dexterity, -1),
    //             (Constitution, 2),
    //             (Intelligence, 1),
    //             (Psyche, -3)
    //         ].into_iter().collect(),
    //         Sarducaan => vec![
    //             (Strength, -2),
    //             (Dexterity, 0),
    //             (Constitution, 2),
    //             (Intelligence, -1),
    //             (Psyche, 3)
    //         ].into_iter().collect(),
    //         Khurite => vec![
    //             (Strength, 1),
    //             (Dexterity, 2),
    //             (Constitution, 4),
    //             (Intelligence, -5),
    //             (Psyche, 1),
    //         ].into_iter().collect(),
    //         Kallard => vec![
    //             (Strength, 2),
    //             (Dexterity, -1),
    //             (Constitution, 5),
    //             (Intelligence, -7),
    //             (Psyche, 1)
    //         ].into_iter().collect(),            

    //         Veela => vec![
    //             (Strength, -3),
    //             (Dexterity, 5),
    //             (Constitution, 1),
    //             (Intelligence, -2),
    //             (Psyche, 3),
    //         ].into_iter().collect(),
    //         Sheevra => vec![
    //             (Strength, -4),
    //             (Dexterity, 4),
    //             (Constitution, 1),
    //             (Intelligence, 0),
    //             (Psyche, 4),                
    //         ].into_iter().collect(),

    //         Blainn => vec![
    //             (Strength, 4),
    //             (Dexterity, -2),
    //             (Constitution, 9),
    //             (Intelligence, 1),
    //             (Psyche, -1),
    //         ].into_iter().collect(),
    //         Huergar => vec![
    //             (Strength, 4),
    //             (Dexterity, -2),
    //             (Constitution, 7),
    //             (Intelligence, 2),
    //             (Psyche, 0)
    //         ].into_iter().collect(),

    //         Thursar => vec![
    //             (Strength, 5),
    //             (Dexterity, -1),
    //             (Constitution, 6),
    //             (Intelligence, -6),
    //             (Psyche, -10),
    //         ].into_iter().collect(),
    //     }
    // }

    // pub fn attributes(&self) -> EnumMap<Attribute, i8> {
    //     use Race::*;
    //     use Attribute::*;
    //     match self {
    //         Tindremen => EnumMap::default(),
    //         Sidoian => vec![
    //             (Strength, 2),
    //             (Dexterity, -1),
    //             (Constitution, 2),
    //             (Intelligence, 1),
    //             (Psyche, -3)
    //         ].into_iter().collect(),
    //         Sarducaan => vec![
    //             (Strength, -2),
    //             (Dexterity, 0),
    //             (Constitution, 2),
    //             (Intelligence, -1),
    //             (Psyche, 3)
    //         ].into_iter().collect(),
    //         Khurite => vec![
    //             (Strength, 1),
    //             (Dexterity, 2),
    //             (Constitution, 4),
    //             (Intelligence, -5),
    //             (Psyche, 1),
    //         ].into_iter().collect(),
    //         Kallard => vec![
    //             (Strength, 2),
    //             (Dexterity, -1),
    //             (Constitution, 5),
    //             (Intelligence, -7),
    //             (Psyche, 1)
    //         ].into_iter().collect(),

    //         Veela => EnumMap::default(),
    //         Sheevra => vec![
    //             (Strength, -1),
    //             (Dexterity, -1),
    //             (Constitution, 0),
    //             (Intelligence, 2),
    //             (Psyche, 1),
    //         ].into_iter().collect(),

    //         Blainn => EnumMap::default(),
    //         Huergar => vec![
    //             (Strength, 0),
    //             (Dexterity, 0),
    //             (Constitution, -2),
    //             (Intelligence, 1),
    //             (Psyche, 1)
    //         ].into_iter().collect(),

    //         Thursar => EnumMap::default(),
    //     }
    // }

    pub fn min_height(&self) -> i8 {
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

    pub fn max_height(&self) -> i8 {
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

pub fn available_parents(clade: Clade, parent: Parent) -> Vec<Race> {
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