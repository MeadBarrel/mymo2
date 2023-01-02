use serde::{Serialize, Deserialize};
use super::Attribute;

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


impl Race {

    /// Return attribute cap modifier for this race, assuming Tindremen to be
    /// the default
    pub fn attribute_cap(&self, attribute: Attribute) -> i8 {
        use Race::*;
        use Attribute::*;

        match self {
            Tindremen => match attribute {
                Strength => 88,
                Dexterity => 86,
                Constitution => 82,
                Intelligence => 99,
                Psyche => 89
            },
            Sidoian => match attribute {
                Strength => 96,
                Dexterity => 82,
                Constitution => 90,
                Intelligence => 103,
                Psyche => 77,
            }
            Sarducaan => match attribute {
                Strength => 80,
                Dexterity => 86,
                Constitution => 90,
                Intelligence => 95,
                Psyche => 101
            },
            Khurite => match attribute {
                Strength => 92,
                Dexterity => 94,
                Constitution =>  98,
                Intelligence =>  79,
                Psyche => 93,
            },
            Kallard => match attribute {
                Strength => 96,
                Dexterity => 82,
                Constitution => 102,
                Intelligence => 71,
                Psyche => 93
            },            

            Veela => match attribute {
                Strength => 76,
                Dexterity => 106,
                Constitution =>  86,
                Intelligence =>  91,
                Psyche => 101,
            },
            Sheevra => match attribute {
                Strength => 72,
                Dexterity => 102,
                Constitution => 86,
                Intelligence => 99,
                Psyche => 105,                
            },

            Blainn => match attribute {
                Strength => 104,
                Dexterity => 78,
                Constitution => 118,
                Intelligence => 103,
                Psyche => 85,
            },

            Huergar => match attribute {
                Strength => 104,
                Dexterity => 78,
                Constitution =>  110,
                Intelligence =>  107,
                Psyche => 89
            },

            Thursar => match attribute {
                Strength => 108,
                Dexterity => 82,
                Constitution => 106,
                Intelligence => 71,
                Psyche => 49,
            },            
        }
    }

    /// Return minimum height modifier for this race, assuming tindremene to be
    /// the default
    pub fn min_height(&self) -> i32 {
        use Race::*;
        match self {
            Tindremen => 166,
            Sidoian => 166,
            Sarducaan => 162,
            Khurite => 162,
            Kallard => 174,

            Veela => 158,
            Sheevra => 154,

            Blainn => 154,
            Huergar => 150,

            Thursar => 178,
        }
    }

    /// Return maximum height modifier for this race, assuming tindremene to be
    /// the default
    pub fn max_height(&self) -> i32 {
        use Race::*;
        match self {
            Tindremen => 199,
            Sidoian => 195,
            Sarducaan => 183,
            Khurite => 179,
            Kallard => 207,

            Veela => 179,
            Sheevra => 175,

            Blainn => 171,
            Huergar => 167,

            Thursar => 215,
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
