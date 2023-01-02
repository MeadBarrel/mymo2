use super::Attribute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightClass {
    Skeletal,
    Bony,
    Skinny,
    Underweight,
    Lean,
    Fit,
    Stout,
    Overweight,
    Fat,
    Bulging,
    Obese
}

impl WeightClass {
    pub fn name(self) -> String {
        use WeightClass::*;
        match self {
            Skeletal => "Skeletal",
            Bony => "Bony",
            Skinny => "Skinny",
            Underweight => "Underweight",
            Lean => "Lean",
            Fit => "Fit",
            Stout => "Stout",
            Overweight => "Overweight",
            Fat => "Fat",
            Bulging => "Bulging",
            Obese => "Obese",
        }.to_string()
    }

    pub fn attribute_modifier(self, attribute: Attribute) -> i32 {
        use WeightClass::*;
        use Attribute::*;

        match self {
            Skeletal => match attribute {
                Strength => -40,
                Dexterity => 0,
                Constitution => -40,
                Intelligence => 0,
                Psyche => -10,
            },
            Bony => match attribute {
                Strength => -30,
                Dexterity => 5,
                Constitution => -30,
                Intelligence => 10,
                Psyche => -5
            },
            Skinny => match attribute {
                Strength => -20,
                Dexterity => 5,
                Constitution => -15,
                Intelligence => 5,
                Psyche => 0
            },
            Underweight => match attribute {
                Strength => -10,
                Dexterity => 8,
                Constitution => -5,
                Intelligence => 0,
                Psyche => 5
            },
            Lean => match attribute {
                Strength => -5,
                Dexterity => 6,
                Constitution | Intelligence | Psyche => 0,
            },
            Fit => 0,
            Stout => match attribute {
                Strength => 5,
                Dexterity => -5,
                Constitution | Intelligence | Psyche => 0,
            },
            Overweight => match attribute {
                Dexterity => -10,
                Strength | Constitution | Intelligence | Psyche => 0,
            },
            Fat => match attribute {
                Strength => -5,
                Dexterity => -16,
                Constitution => -5,
                Intelligence => 5,
                Psyche => 5
            },
            Bulging => match attribute {
                Strength => -15,
                Dexterity => -22,
                Constitution => -15,
                Intelligence => 10,
                Psyche => 10,
            },
            Obese => match attribute {
                Strength => -30,
                Dexterity => -42,
                Constitution => -30,
                Intelligence => 20,
                Psyche => 20
            }
        }
    }
}