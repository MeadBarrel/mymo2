use std::collections::HashMap;
use eframe::{egui::{Ui, TextureHandle, Widget, ImageButton, Response, plot::Text}, glow::Texture};


pub struct SimpleImageButton<'a> {
    images: &'a HashMap<String, TextureHandle>,
    name: &'a str,
}

impl<'a> SimpleImageButton<'a> {
    pub fn new(images: &'a HashMap<String, TextureHandle>, name: &'a str) -> Self {
        Self {
            images, name
        }
    }
}

impl<'a> Widget for SimpleImageButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        match self.images.get(self.name) {
            Some(image) => ui.add(ImageButton::new(image.id(), image.size_vec2())),
            None => ui.heading(format!("Something went wrong: could not find {} in resources", self.name))
        }
    }
}

pub trait WithSimpleImageButton {
    fn simple_image_button<'a>(&mut self, images: &'a HashMap<String, TextureHandle>, name: &'a str) -> Response;
}

impl WithSimpleImageButton for Ui {
    fn simple_image_button<'a>(&mut self, images: &'a HashMap<String, TextureHandle>, name: &'a str) -> Response {
        self.add(
            SimpleImageButton::new(images, name)
        )
    }
}
