use crate::storage::IMAGES;
use eframe::egui::{Ui, Widget, ImageButton, Response};


pub struct SimpleImageButton<'a> {
    name: &'a str,
}

impl<'a> SimpleImageButton<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name
        }
    }
}

impl<'a> Widget for SimpleImageButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        match IMAGES.get().get(self.name) {
            Some(image) => ui.add(ImageButton::new(image.texture_id(ui.ctx()), image.size_vec2())),
            None => ui.heading(format!("Something went wrong: could not find {} in resources", self.name))
        }
    }
}

pub trait WithSimpleImageButton {
    fn simple_image_button<'a>(&mut self, name: &'a str) -> Response;
}

impl WithSimpleImageButton for Ui {
    fn simple_image_button<'a>(&mut self, name: &'a str) -> Response {
        self.add(
            SimpleImageButton::new(name)
        )
    }
}
