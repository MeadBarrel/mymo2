use mymo::model::Character;

use crate::{components::PropComponent, containers::mymo_frame_defaults};

use super::slider::SliderFrame;

pub struct AgeSlider;

impl PropComponent for AgeSlider {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        let mut container = mymo_frame_defaults();
        let min_age = item.min_age();
        let max_age = item.max_age();

        let mut slider_frame = SliderFrame::new()
            .label("Age")
            .value_fmt(item.age.to_string())
            .min(min_age)
            .max(max_age);
        
        container.show(ui, |ui| {
            slider_frame.add(frame, ui, &mut item.age);
        });
        
    }
}