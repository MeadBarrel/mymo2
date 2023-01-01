mod raceselect;
mod attribute;
mod parents;

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
    parents_editor: parents::ParentsEditor,
}

impl CharacterEditor {
    pub fn new(id: SuffixedId) -> Self {
        Self {
            parents_editor: parents::ParentsEditor::new(id.derive("parents_editor")),
            id,
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
                            self.parents_editor.add(ui, item)
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
