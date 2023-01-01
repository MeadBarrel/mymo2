mod id;
mod app;
mod components;
mod error;
mod widgets;

pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(app::App::new(cc))));
}