use eframe::epaint::ahash::HashMapExt;
use std::collections::HashMap;
use eframe::egui::*;
use crate::components::{AppComponent, CharacterEditor, PropWrapper};
use crate::id::SuffixedId;
use std::rc::Rc;
use std::cell::Cell;
use crate::error::{Error, Result};



#[derive(Default)]
pub struct App {
    character: Rc<Cell<mymo::model::Character>>,
    id: SuffixedId,
    pub images: HashMap<String, TextureHandle>,
}


impl App {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        let images = load_images(&ctx.egui_ctx, "./resources").unwrap();
        println!("keys: {:?}", images.keys());
        Self {
            images,
            ..Default::default()
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {

        let mut style = Style::default();
        use TextStyle::*;
        use eframe::egui::FontFamily::Proportional;

        style.text_styles = [
            (Heading, FontId::new(24.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(12.0, Proportional)),
        ].into();

        ctx.set_style(style);

        CentralPanel::default().show(ctx, |ui| {
            PropWrapper::new(
                CharacterEditor::new(self.id.derive("character_editor")), self.character.clone()
            ).add(self, ui);
        });
    }
}

pub fn load_images(ctx: &Context, path: &str) -> Result<HashMap<String, TextureHandle>> {
    let mut result = HashMap::new();

    let entries = std::fs::read_dir(path)
        .map_err(|e| Error::StartupFailed(format!("Cannot open folder {} ({})", path, e)))?;

    for entry in entries {
        let maybe_file_path = 
            entry.map_err(|_| Error::StartupFailed("Cannot load resource".to_string()))?;        
        let file_path = maybe_file_path.path();

        if !file_path.is_file() { continue; };
        let file_name = maybe_file_path.file_name();

        if let Some(ext) = file_path.extension() {
            if ext != "png" { continue; }
            //let image = ctx.load_texture(file_name.to_str().unwrap().to_string(), image, options)
            let image = load_image(ctx, file_path.to_str().unwrap())?;
            result.insert(file_name.to_str().unwrap().to_string(), image);
        }
        
    }

    Ok(result)
}

fn load_image(ctx: &Context, path: &str) -> Result<TextureHandle> {
    let image = image::io::Reader::open(path)
        .map_err(|e| Error::StartupFailed(format!("Could not load image ({})", e)))?
        .decode()
        .map_err(|_| Error::StartupFailed("Could not decode image".to_string()))?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let color_image = eframe::egui::ColorImage::from_rgba_unmultiplied(
    size,
    pixels.as_slice());

    Ok(
        ctx.load_texture(
            path.to_string(), 
            ImageData::Color(color_image), 
            TextureOptions::default(),
        )
    )

    // Ok(ctx.tex_manager().write().alloc(
    //     path.to_string(),
    //     ImageData::Color(color_image), 
    //     TextureOptions::default(),
    // ))
}