use mymo::model::Character;
use crate::components::PropComponent;
use super::slider::SliderFrame;

pub struct WeightSlider;

impl PropComponent for WeightSlider {
    type Item = Character;

    fn add(&mut self, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        let weight_class = item.weight_class();
        let mut slider_frame = SliderFrame::new()
            .label("Weight")
            .value_fmt(format!("{}%", item.weight_percent))
            .sub_string(format!("{1} | {0:>3}kg", item.weight_kg(), weight_class.name()))
            .min(50)
            .max(200);
        
        slider_frame.add(ui, &mut item.weight_percent);        
    }
}