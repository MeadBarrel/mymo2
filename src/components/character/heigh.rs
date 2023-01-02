use mymo::model::Character;

use crate::{components::PropComponent, containers::mymo_frame_defaults};
use crate::storage::COLORS;
use super::slider::SliderFrame;

#[derive(Debug)]
pub struct HeightSlider;

impl PropComponent for HeightSlider {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        let mut container = mymo_frame_defaults();
        let min_height = item.min_height();
        let max_height = item.max_height();
        let mut slider_frame = SliderFrame::new()
            .label("Height")
            .value_fmt(format!("{}cm", item.height))
            .sub_string(
                format!("{}-{}", min_height, max_height)
            )
            .min(150)
            .max(230);

        if item.height < min_height || item.height > max_height {
            container = container.fill(COLORS.get().frame_color_impossible)
        }

        container.show(ui, |ui| {
            slider_frame.add(frame, ui, &mut item.height)
        });
    }
}