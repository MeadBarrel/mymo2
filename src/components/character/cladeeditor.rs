use mymo::{model::{Character, Clade, Sex}, strum::IntoEnumIterator};
use eframe::egui;
use mymo::strum::EnumIter;

use crate::components::PropComponent;

#[derive(Debug)]
pub struct CladeEditor;

impl PropComponent for CladeEditor {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Self::Item) {
        ui.horizontal(|ui| {
            ui.label("Clade: ");
            ui.menu_button(item.clade.name(), |ui| {
                Clade::iter().for_each(|clade| {
                    if ui.button(clade.name()).clicked() {
                        if item.clade != clade {
                            item.clade = clade;
                            item.parents = clade.default_parents();
                        }
                        ui.close_menu();
                    }
                })
            });

            ui.add_space(10.);
            ui.separator();
            ui.add_space(10.);

            ui.label("Sex:");
            ui.menu_button(item.sex.name(), |ui| {
                Sex::iter().for_each(|sex| {
                    if ui.button(sex.name()).clicked() {
                        item.sex = sex;
                        ui.close_menu();
                    }
                })
            })
        });
    }
}