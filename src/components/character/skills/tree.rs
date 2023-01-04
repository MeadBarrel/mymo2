use crate::components::PropComponent;
use crate::storage::SKILLS;
use crate::id::SuffixedId;
use super::branch::Branch;
use eframe::egui;
use mymo::indexmap::IndexMap;
use mymo::model::{Skill, Character, SkillTree};
use super::skill::SkillEditor;



#[derive(Debug)]
pub struct SkillTreeEditor {
    id: SuffixedId,
    tree: Vec<Branch>,
}

impl SkillTreeEditor {
    pub fn new(id: SuffixedId, tree: Vec<Branch>) -> Self {
        Self { 
            id,
            tree,
            //profession_tree: build_skill_tree(None, SKILLS.get(), SkillTree::Profession),
        }
    }
  
}

impl PropComponent for SkillTreeEditor {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Self::Item) {
        self.tree.iter().for_each(|branch| {
            SkillEditor::new(
                self.id.derive(&format!("_skill_{}", branch.skill.name)), 
                branch
            )
            .show_description(true)
            .add(frame, ui, item);
        });
    }
}
