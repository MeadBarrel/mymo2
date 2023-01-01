mod raceselect;
mod attribute;
mod parents;
mod cladeeditor;
mod slider;
mod heightslider;
mod weightslider;

use mymo::model::Character;
use mymo::model::Attribute;
use mymo::strum::IntoEnumIterator;
use mymo::model::Parent;
use crate::id::SuffixedId;

use self::cladeeditor::CladeEditor;

use super::PropComponent;
use crate::containers::frame;
use eframe::egui::{Ui, ScrollArea, SidePanel, CentralPanel};

#[derive(Debug)]
pub struct CharacterEditor {
    id: SuffixedId,
    clade_editor: cladeeditor::CladeEditor,
    parents_editor: parents::ParentsEditor,
}

impl CharacterEditor {
    pub fn new(id: SuffixedId) -> Self {
        Self {
            clade_editor: CladeEditor,
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
                            self.clade_editor.add(ui, item)
                        });
                        frame(ui, |ui| {
                            self.parents_editor.add(ui, item)
                        });
                        heightslider::HeightSlider.add(ui, item);
                        frame(ui, |ui| {
                            weightslider::WeightSlider.add(ui, item)
                        });
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
                        Attribute::iter().for_each(|attribute| {
                            attribute::AttributeFrame::new(attribute).add(ui, item)
                        })    
                    })
                });
        });

    }
}
