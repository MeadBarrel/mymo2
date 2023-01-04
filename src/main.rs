mod id;
mod app;
mod components;
mod containers;
mod widgets;
mod storage;

use error_stack::{Result, ResultExt, IntoReport};
use tracing::info;

#[derive(Debug, thiserror::Error)]
#[error("Startup failed")]
pub struct StartupError;

pub fn main() -> Result<(), StartupError> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_env_filter("mymo3=debug")
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .into_report()
        .change_context(StartupError)
        .attach_printable_lazy(|| "Cannot initialize logging")?;

    info!("Starting up");

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