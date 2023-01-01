use mymo::model::Character;

use crate::{components::PropComponent, containers::frame};

use super::slider::SliderFrame;

#[derive(Debug)]
pub struct HeightSlider;

impl PropComponent for HeightSlider {
    type Item = Character;

    fn add(&mut self, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        let mut slider_frame = SliderFrame::new()
            .label("Height")
            .value_fmt(format!("{}cm", item.height))
            .min(150)
            .max(230);
        
        slider_frame.add(ui, &mut item.height);
    }
}