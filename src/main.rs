mod id;
mod app;
mod components;
mod containers;
mod widgets;
mod storage;

use error_stack::{Result, ResultExt, IntoReport};

#[derive(Debug, thiserror::Error)]
#[error("Startup failed")]
pub struct StartupError;

pub fn main() -> Result<(), StartupError> {
    let native_options = eframe::NativeOptions::default();

    let images = storage::load_images("./resources").change_context(StartupError)?;
    let skills = mymo::load_skills("./predef/skills.yaml")
        .into_report()
        .change_context(StartupError)
        .attach_printable_lazy(|| "Could not load skills")?;

    storage::IMAGES.set(images);
    storage::COLORS.set(storage::Colors::default());
    storage::SKILLS.set(skills);

    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(app::App::new(cc))));

    Ok(())
}