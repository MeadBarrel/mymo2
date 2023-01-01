use mymo::model::{Character, Attribute};
use eframe::egui::{Ui, Align, Layout, Slider};
use crate::containers::frame;

pub(super) fn attribute_frame(ui: &mut Ui, character: &mut Character, attribute: Attribute) {
    let attribute_cap = character.attribute_cap(attribute);
    frame(ui, |ui| {
        ui.vertical(|ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add_space(5.);
                    ui.heading(&attribute.name());
                });
            
                ui.with_layout(Layout::top_down(Align::Max), |ui| {
                        ui.label(attribute_cap.to_string());
                        ui.label(character.attributes[attribute].to_string());
                })
            });
            ui.add_space(5.);
            ui.add(
                Slider::new(&mut character.attributes[attribute], 10..=attribute_cap)
                    .show_value(false)
            );
        })    
    });
}