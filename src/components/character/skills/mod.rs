mod branch;
mod skill;
mod tree;

use eframe::egui::{self, ScrollArea};
use mymo::model::{Character, SkillTree};
use tree::SkillTreeEditor;
use crate::{id::SuffixedId, components::PropComponent, storage::SKILLS, containers::box_container};

#[derive(Debug)]
pub struct CharacterSkillsEditor {
    id: SuffixedId,
    action_tree: SkillTreeEditor,
    profession_tree: SkillTreeEditor,

    min_left_panel_size: f32,
    min_right_panel_size: f32,
    left_panel_ratio: f32,

}

impl CharacterSkillsEditor {
    pub fn new(id: SuffixedId) -> Self {
        let action_tree = branch::build_skill_tree(None, SKILLS.get(), SkillTree::Action);
        let profession_tree = branch::build_skill_tree(None, SKILLS.get(), SkillTree::Profession);

        Self { 
            action_tree: SkillTreeEditor::new(id.derive("action_tree"), action_tree),
            profession_tree: SkillTreeEditor::new(id.derive("profession_tree"), profession_tree),
            id,

            min_left_panel_size: 400.,
            min_right_panel_size: 400.,
            left_panel_ratio: 0.5,
        }
    }

    fn wide_layout(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Character) {
        let available_width = ui.available_width();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width_range(
                    self.min_left_panel_size..=(available_width * self.left_panel_ratio));
                self.left_panel(frame, ui, item);
            });
            ui.vertical(|ui| {
                ui.set_width_range(
                    self.min_right_panel_size..=(available_width * (1.-self.left_panel_ratio)));
                self.right_panel(frame, ui, item);
            })
        });
    }

    fn small_layout(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Character) {
        ui.vertical(|ui| {
            ui.set_width(ui.available_width());
            self.left_panel(frame, ui, item);
            self.right_panel(frame, ui, item);
        });
    }

    fn left_panel(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Character) {
        box_container(ui, |ui| {
            ui.collapsing("Action Skills", |ui| {
                self.action_tree.add(frame, ui, item)
            });
            ui.collapsing("Profession Skills", |ui| {
                self.profession_tree.add(frame, ui, item)
            })
        });
    }

    fn right_panel(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Character) {
    }
}

impl PropComponent for CharacterSkillsEditor {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        let available_width = ui.available_width();
        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            if available_width < self.min_left_panel_size + self.min_right_panel_size {
                self.small_layout(frame, ui, item)
            } else {
                self.wide_layout(frame, ui, item)
            }
        });
    }
}