use mymo::model::Character;

use crate::{id::SuffixedId, components::PropComponent};

#[derive(Debug)]
pub struct CharacterSkillsEditor {
    id: SuffixedId,
}

impl CharacterSkillsEditor {
    pub fn new(id: SuffixedId) -> Self {
        Self { id }
    }
}

impl PropComponent for CharacterSkillsEditor {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        
    }
}