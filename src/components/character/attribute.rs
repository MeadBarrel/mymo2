use mymo::model::{Character, Attribute};
use eframe::{egui::Ui, epaint::Color32};
use crate::components::PropComponent;
use super::slider::SliderFrame;
use crate::containers::mymo_frame_defaults;
use crate::storage::COLORS;

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

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Self::Item) {
        let modifier = item.attribute_modifier(self.attribute);
        let raw_attribute = item.attributes[self.attribute];
        let attribute_min = item.attribute_min(self.attribute);
        let mut container = mymo_frame_defaults();

        let attribute_cap = item.attribute_cap(self.attribute);
        let slider_frame = SliderFrame::new()
            .label(self.attribute.name())
            .value_fmt(
                format!(
                    "{} ({})",
                    item.attribute(self.attribute),
                    if modifier > 0 { format!("+{}", modifier) }
                    else { modifier.to_string() },
                )
            )
            .sub_string(format!("{}-{}", attribute_min, attribute_cap))
            .min(attribute_min)
            .max(140);

        if item.attributes[self.attribute] > attribute_cap {
            container = container.fill(COLORS.get().frame_color_impossible)
        }

        if modifier < 0 
            && raw_attribute + modifier < attribute_min + 1 
            && raw_attribute > attribute_min 
        {
            container = container.fill(COLORS.get().frame_color_ineffective)
        }

        container.show(ui, |ui| {
            slider_frame
                .show_value(true)
                .add(frame, ui, &mut item.attributes[self.attribute]);     
        });
        
    }
}
