use std::vec;
use serde::{Serialize, Deserialize};
use enum_map::{Enum, EnumMap};
use strum::EnumIter;
use super::Race;


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
                (First, Risar), (Second, Risar), (Third, Tindremen), (Fourth, Tindremen)]
                .into_iter().collect()
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
            Parent::First | Parent::Second => vec![Risar],
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