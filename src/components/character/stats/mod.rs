mod race;
mod attribute;
mod parents;
mod clade;
mod slider;
mod height;
mod weigh;
mod age;

use mymo::model::Character;
use mymo::model::Attribute;
use mymo::strum::IntoEnumIterator;
use crate::id::SuffixedId;

use self::clade::CladeEditor;

use super::PropComponent;
use crate::containers::box_container;
use eframe::egui::{Ui, ScrollArea};

#[allow(unused)]
#[derive(Debug)]
pub struct CharacterStatsEditor {
    id: SuffixedId,
    clade_editor: clade::CladeEditor,
    parents_editor: parents::ParentsEditor,
    left_min_size: f32,
    central_min_size: f32,
    right_min_size: f32,
    left_ratio: f32,
    central_ratio: f32,
    right_ratio: f32,
}

impl CharacterStatsEditor {
    pub fn new(id: SuffixedId) -> Self {
        Self {
            clade_editor: CladeEditor,
            parents_editor: parents::ParentsEditor::new(id.derive("parents_editor")),
            id,
            left_min_size: 400.,
            central_min_size: 400.,
            right_min_size: 400.,
            left_ratio: 0.35,
            central_ratio: 0.3,
            right_ratio: 0.35,
        }
    }

    fn wide_layout(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Character) {
        let available_width = ui.available_width();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width_range(self.left_min_size..=(available_width * self.left_ratio));
                self.left_panel(frame, ui, item);
            });
            ui.vertical(|ui| {
                ui.set_width_range(self.central_min_size..=(available_width * self.central_ratio));
                self.central_panel(frame, ui, item);
            });
            ui.vertical(|ui| {
                ui.set_width_range(self.right_min_size..=(available_width * self.right_ratio));
                self.right_panel(frame, ui, item);
            });            
        });
    }

    fn mid_layout(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Character) {
        let available_width = ui.available_width();
        let normalized_ratos = normalize_vec(&[self.left_ratio, self.central_ratio]);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width_range(self.left_min_size..=(available_width * normalized_ratos[0]));
                    self.left_panel(frame, ui, item);
                });
                ui.vertical(|ui| {
                    ui.set_width_range(self.central_min_size..=(available_width * normalized_ratos[1]));
                    self.central_panel(frame, ui, item);
                });    
            });
            ui.vertical(|ui| {
                ui.set_width(available_width);
                self.right_panel(frame, ui, item);
            });            
        });
    }

    fn small_layout(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Character) {
        ui.vertical(|ui| {
            ui.set_width(ui.available_width());
            self.left_panel(frame, ui, item);
            self.central_panel(frame, ui, item);
            self.right_panel(frame, ui, item);
        });
    }

    fn left_panel(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Character) {
        ui.add_space(5.);
        box_container(ui, |ui| {
            self.clade_editor.add(frame, ui, item)
        });
        box_container(ui, |ui| {
            self.parents_editor.add(frame, ui, item)
        });
        age::AgeSlider.add(frame, ui, item);
        height::HeightSlider.add(frame, ui, item);
        box_container(ui, |ui| {
            weigh::WeightSlider.add(frame, ui, item)
        });        
    }

    fn central_panel(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Character) {
        ui.scope(|ui| {                        
            Attribute::iter().for_each(|attribute| {
                attribute::AttributeFrame::new(attribute).add(frame, ui, item)
            })
        });
    }

    fn right_panel(&mut self, _: &mut eframe::Frame, _: &mut Ui, _: &mut Character) {
    }
}

impl PropComponent for CharacterStatsEditor {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Self::Item) {
        let available_width = ui.available_width();
        ScrollArea::new([false, true]).show(ui, |ui| {
            if available_width < self.left_min_size + self.right_min_size {
                self.small_layout(frame, ui, item)
            } else if available_width < self.left_min_size + self.right_min_size + self.central_min_size {
                self.mid_layout(frame, ui, item)
            } else {
                self.wide_layout(frame, ui, item)
            }
        });
    }
}

fn normalize_vec<const N: usize>(vec: &[f32; N]) -> Vec<f32> {
    let total: f32 = vec.iter().sum();
    vec.iter().map(|x| x / total).collect()
}