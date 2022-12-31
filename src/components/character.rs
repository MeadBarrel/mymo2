use mymo::model::Character;
use mymo::model::Attribute;
use mymo::strum::IntoEnumIterator;
use crate::app::App;
use super::PropComponent;
use eframe::{egui::
    {self, Layout, Ui, InnerResponse}, 
    epaint::{Color32, Shadow}, 
    emath::Align
};

#[derive(Debug, Default)]
pub struct CharacterEditor {}

impl PropComponent for CharacterEditor {
    type Context = App;
    type Item = Character;

    fn add(&mut self, _: &mut Self::Context, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            ui.with_layout(Layout::top_down_justified(eframe::emath::Align::Min), |ui| {   
                ui.set_width(ui.available_width() * 0.3);
                ui.spacing_mut().slider_width = ui.available_width() * 0.9;            
                Attribute::iter().for_each(|attribute| {
                    frame(ui, |ui| {
                        attribute_frame(ui, item, attribute)
                    });                
                })
            });
    
        });
    }
}

fn attribute_frame(ui: &mut Ui, character: &mut Character, attribute: Attribute) -> InnerResponse<()> {
    let attribute_cap = character.attribute_cap(attribute);
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
            egui::Slider::new(&mut character.attributes[attribute], 10..=attribute_cap)
                .show_value(false)
        );
    })    
}

fn frame<R>(ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    egui::Frame::none()
        .fill(Color32::from_additive_luminance(25))
        .outer_margin(10.)
        .inner_margin(20.)
        .shadow(Shadow::small_light())
        .rounding(4.0)
        .show(ui, |ui| {                    
            content(ui)
        })
}
