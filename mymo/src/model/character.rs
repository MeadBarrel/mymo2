use enum_map::EnumMap;
use super::{attribute::Attribute, Race, Parent, Clade, Sex};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub clade: Clade,
    pub sex: Sex,
    pub weight_percent: i32,
    pub height: i32,
    pub parents: EnumMap<Parent, Race>,
    pub attributes: EnumMap<Attribute, i32>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            clade: Default::default(),
            sex: Default::default(),
            weight_percent: 50,
            height: 150,
            parents: Default::default(),
            attributes: Default::default(),
        }
    }
}

impl Character {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clade(mut self, clade: Clade) -> Self {
        self.clade = clade;
        self
    }

    pub fn sex(mut self, sex: Sex) -> Self {
        self.sex = sex;
        self
    }

    pub fn weight_percent(mut self, weight_percent: i32) -> Self {
        self.weight_percent = weight_percent;
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    pub fn parents(mut self, parents: [Race; 4]) -> Self {
        self.parents[Parent::First] = parents[0];
        self.parents[Parent::Second] = parents[1];
        self.parents[Parent::Third] = parents[2];
        self.parents[Parent::Fourth] = parents[3];
        self
    }

    pub fn strength(mut self, strength: i32) -> Self {
        self.attributes[Attribute::Strength] = strength;
        self
    }

    pub fn dexterity(mut self, dexterity: i32) -> Self {
        self.attributes[Attribute::Dexterity] = dexterity;
        self
    }

    pub fn constitution(mut self, constitution: i32) -> Self {
        self.attributes[Attribute::Constitution] = constitution;
        self
    }

    pub fn intelligence(mut self, intelligence: i32) -> Self {
        self.attributes[Attribute::Intelligence] = intelligence;
        self
    }

    pub fn psyche(mut self, psyche: i32) -> Self {
        self.attributes[Attribute::Psyche] = psyche;
        self
    }

    /// Get attribute cap modifier for this character at age 18, height 150cm, 
    /// 100% weight, assuming full tindremene to be the default.
    pub fn attribute_cap(&self, attribute: Attribute) -> i32 {
        let result = attribute.default_cap() as i32 
            + self.parents.values().map(|x| x.attribute_cap(attribute)).sum::<i8>() as i32;
        result.max(0)
    }

    pub fn attribute_min(&self, attribute: Attribute) -> i32 {
        10
    }

    /// Get total attribute modifier
    pub fn attribute_modifier(&self, attribute: Attribute) -> i32 {
        self.weight_class().attribute_modifier(attribute)
    }

    /// Get final value of an attribute
    pub fn attribute(&self, attribute: Attribute) -> i32 {
        let modifier = self.attribute_modifier(attribute);
        (self.attributes[attribute] + modifier)
            .clamp(self.attribute_min(attribute), self.attribute_cap(attribute) + modifier)
    }

    /// Get weight class
    pub fn weight_class(&self) -> WeightClass {
        use WeightClass::*;
        if self.weight_percent < 53 { return Skeletal };
        if self.weight_percent < 66 { return Bony };
        if self.weight_percent < 75 { return Skinny };
        if self.weight_percent < 83 { return Underweight };
        if self.weight_percent < 93 { return Lean };
        if self.weight_percent < 111 { return Fit };
        if self.weight_percent < 131 { return Stout };
        if self.weight_percent < 151 { return Overweight };
        if self.weight_percent < 171 { return Fat };
        if self.weight_percent < 196 { return Bulging };
        
        Obese
    }

    /// Get weight in kilograms
    pub fn weight_kg(&self) -> i32 {
        let w = self.weight_percent as f32;
        let h = self.height as f32;
        let result = w * h * h * 0.000021251 + 10.;
        result as i32
    }
}

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

#[cfg(test)]
pub mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case(50, 150, 33)]
    #[case(100, 150, 57)]
    #[case(200, 150, 105)]
    #[case(50, 200, 52)]
    #[case(100, 200, 95)]
    #[case(200, 200, 180)]
    #[case(50, 230, 66)]
    #[case(100, 230, 122)]
    #[case(200, 230, 234)]     
    #[case(181, 166, 115)]   
    pub fn test_weight_kg(#[case] wp: i32, #[case] h: i32, #[case] expected: i32) {
        let character = Character::new()
            .weight_percent(wp)
            .height(h);
        let actual = character.weight_kg();
        assert_eq!(actual, expected);
    }
}