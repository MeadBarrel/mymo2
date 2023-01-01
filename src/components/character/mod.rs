mod raceselect;
mod attribute;

use mymo::model::Character;
use mymo::model::Attribute;
use mymo::strum::IntoEnumIterator;
use mymo::model::Parent;
use crate::id::SuffixedId;
use super::PropComponent;
use crate::containers::frame;
use eframe::egui::{Ui, ScrollArea, SidePanel, CentralPanel};

#[derive(Debug)]
pub struct CharacterEditor {
    id: SuffixedId,
    parent_buttons: Vec<raceselect::RaceSelectButton>,
}

impl CharacterEditor {
    pub fn new(id: SuffixedId) -> Self {
        let parent_buttons = Parent::iter().enumerate().map(|(i, parent)| {
            raceselect::RaceSelectButton::new(id.derive(&format!("select_parent_{i}")), parent)
        }).collect();
        Self {
            id,
            parent_buttons,
        }
    }
}

impl PropComponent for CharacterEditor {
    type Item = Character;

    fn add(&mut self, ui: &mut Ui, item: &mut Self::Item) {
        let side_panel_width = ui.available_width() * 0.35;
        ScrollArea::new([false, true]).show(ui, |ui| {
            SidePanel::left(self.id.derive("side_panel"))
                .resizable(false)
                .exact_width(side_panel_width)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.);
                        frame(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.heading("Race:");
                                    self.parent_buttons.iter_mut().for_each(|parent_button| {
                                        ui.add_space(10.);
                                        parent_button.add(ui, item)
                                    });
                                })
    
                            });
                            
                        })
                    });
                });

        
            SidePanel::right(self.id.derive("left_panel"))
                .exact_width(side_panel_width)
                .resizable(false)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                   
                });

            
            CentralPanel::default()
                .show_inside(ui, |ui| {
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = ui.available_width() * 0.9;
                        Attribute::iter().for_each(|attribute| {
                            attribute::attribute_frame(ui, item, attribute)
                        })    
                    })
                });
        });

    }
}
