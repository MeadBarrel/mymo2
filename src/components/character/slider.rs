use std::marker::PhantomData;

use eframe::egui::{Ui, emath::Numeric, Align, Layout, Slider};
use mymo::model::Character;
use super::frame;

use crate::components::PropComponent;

#[derive(Debug, Clone, Copy)]
pub enum SliderFrameColor {
    Normal,
    Ineffective,
    Impossible,
}

pub struct SliderFrame<Num> {
    label: Option<String>,
    value_fmt: Option<String>,
    sub_string: Option<String>,
    min: Num,
    max: Num,
    step: Option<Num>,
    
}

impl<Num> SliderFrame<Num> 
    where 
        Num: Numeric
{
    pub fn new() -> Self {
        Self {
            label: None,
            value_fmt: None,
            sub_string: None,
            min: Num::MIN,
            max: Num::MAX,
            step: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value_fmt(mut self, value_fmt: impl Into<String>) -> Self {
        self.value_fmt = Some(value_fmt.into());
        self
    }

    pub fn sub_string(mut self, sub_string: impl Into<String>) -> Self {
        self.sub_string = Some(sub_string.into());
        self
    }

    pub fn min(mut self, min: Num) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: Num) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: Num) -> Self {
        self.step = Some(step);
        self
    }
}

impl<Num> PropComponent for SliderFrame<Num> 
    where 
        Num: Numeric,
{
    type Item = Num;

    fn add(&mut self, ui: &mut Ui, item: &mut Self::Item) {
        frame(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
            
                ui.horizontal(|ui| {
                    if let Some(label) = &self.label {
                        ui.vertical(|ui| {
                            ui.add_space(5.);
                            ui.heading(label.as_str());
                        });
                    };
                    ui.with_layout(Layout::top_down(Align::Max), |ui| {
                        if let Some(value_fmt) = &self.value_fmt {
                            ui.label(value_fmt);
                        }
                        if let Some(sub_string) = &self.sub_string {
                            ui.label(sub_string);
                        }
                    });               
                });
                ui.add_space(5.);
                ui.add(
                    Slider::new(item, self.min..=self.max)
                        .show_value(false)
                )
            })
        });
    }
}