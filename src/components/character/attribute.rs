use mymo::model::{Character, Attribute};
use eframe::egui::Ui;
use crate::components::PropComponent;
use super::slider::SliderFrame;

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
        let mut slider_frame = SliderFrame::new()
            .label(self.attribute.name())
            .value_fmt( item.attributes[self.attribute].to_string() )
            .sub_string(format!("10-{}", attribute_cap))
            .min(10)
            .max(140);

        slider_frame.add(ui, &mut item.attributes[self.attribute]);     
    }
}
