use crate::components::PropComponent;
use crate::id::SuffixedId;
use mymo::model::{Parent, Character};
use mymo::strum::IntoEnumIterator;
use super::raceselect::RaceSelectButton;

#[derive(Debug)]
pub(super) struct ParentsEditor {
    parent_buttons: Vec<RaceSelectButton>,    
}

impl ParentsEditor {
    pub fn new(id: SuffixedId) -> Self {
        let parent_buttons = Parent::iter().enumerate().map(|(i, parent)| {
            RaceSelectButton::new(id.derive(&format!("select_parent_{i}")), parent)
        }).collect();
        Self {
            parent_buttons,
        }
    }
}

impl PropComponent for ParentsEditor {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Race:");
                self.parent_buttons.iter_mut().for_each(|parent_button| {
                    ui.add_space(10.);
                    parent_button.add(frame, ui, item)
                });
            })
        });        
    }
}