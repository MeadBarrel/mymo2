use serde::{Serialize, Deserialize};
use super::attribute::Attribute;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, PartialEq, Eq)]
pub enum SkillTree {
    #[default]
    Action,
    Profession,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub name: String,
    #[serde(default="Vec::default")]
    pub parents: Vec<(String, u8)>,
    #[serde(default="Option::default")]
    pub description: Option<String>,
    #[serde(default="Option::default")]
    pub primary_attribute: Option<Attribute>,
    #[serde(default="Option::default")]
    pub secondary_attribute: Option<Attribute>,
    #[serde(default="SkillTree::default")]
    pub tree: SkillTree,
    #[serde(default="bool::default")]
    pub is_primary: bool,
}


impl Skill {
    pub fn new(name: impl Into<String>, type_: SkillTree) -> Self {
        Self {
            name: name.into(),
            tree: type_,
            parents: Vec::default(),
            description: None,
            primary_attribute: None,
            secondary_attribute: None,
            is_primary: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn primary_attribute(mut self, attribute: Attribute) -> Self {
        self.primary_attribute = Some(attribute);
        self
    }

    pub fn secondary_attribute(mut self, attribute: Attribute) -> Self {
        self.secondary_attribute = Some(attribute);
        self
    }

    pub fn primary(mut self, primary: bool) -> Self {
        self.is_primary = primary;
        self
    }

    pub fn has_parent(&self, parent_name: &str) -> bool {
        self.parents.iter().any(|x| x.0 == parent_name)
    }
}