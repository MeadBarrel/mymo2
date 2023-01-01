mod id;
mod app;
mod components;
mod widgets;
mod storage;

use error_stack::{Result, ResultExt};

#[derive(Debug, thiserror::Error)]
#[error("Startup failed")]
pub struct StartupError;

pub fn main() -> Result<(), StartupError> {
    let native_options = eframe::NativeOptions::default();

    let images = storage::load_images("./resources").change_context(StartupError)?;

    storage::IMAGES.set(images);

    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(app::App::new(cc))));

    Ok(())
}