use mymo::model::{Skill, SkillTree};
use mymo::indexmap::IndexMap;

#[derive(Debug)]
pub struct Branch {
    pub skill: Skill,
    pub children: Vec<Branch>
}

pub fn build_skill_tree(parent: Option<Skill>, skills: &IndexMap<String, Skill>, tree: SkillTree) -> Vec<Branch> {
    skills.values()
        .filter(|skill| {
            skill.tree == tree
        })
        .filter(|skill|
            match &parent {
                Some(parent) => skill.has_parent(&parent.name),
                None => skill.parents.is_empty(),
            }
        )
        .map(|skill| {
            Branch {
                skill: skill.clone(),
                children: build_skill_tree(Some(skill.clone()), skills, tree)
            }
        })
        .collect()
}