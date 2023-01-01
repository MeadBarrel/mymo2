use mymo::model::{Character, Attribute};
use eframe::egui::{Ui, Align, Layout, Slider};
use crate::{containers::frame, components::PropComponent};

pub(super) struct AttributeFrame {
    attribute: Attribute,
}

impl AttributeFrame {
    pub fn new(attribute: Attribute) -> Self {
        Self { attribute }
    }
}

impl PropComponent for AttributeFrame {
    type Item = Character;

    fn add(&mut self, ui: &mut Ui, item: &mut Self::Item) {
        let attribute_cap = item.attribute_cap(self.attribute);
            frame(ui, |ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.add_space(5.);
                            ui.heading(&self.attribute.name());
                        });
                    
                        ui.with_layout(Layout::top_down(Align::Max), |ui| {
                                ui.label(attribute_cap.to_string());
                                ui.label(item.attributes[self.attribute].to_string());
                        })
                    });
                    ui.add_space(5.);
                    ui.add(
                        Slider::new(&mut item.attributes[self.attribute], 10..=attribute_cap)
                            .show_value(false)
                    );
                })    
            });        
    }
}
