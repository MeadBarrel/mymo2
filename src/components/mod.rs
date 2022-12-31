mod propwrapper;
mod character;

pub use propwrapper::PropWrapper;
pub use character::CharacterEditor;

use eframe::egui::Ui;

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn add(&mut self, ctx: &mut Self::Context, ui: &mut Ui);
}

pub trait PropComponent {
    type Context;
    type Item;

    #[allow(unused)]
    fn add(&mut self, ctx: &mut Self::Context, ui: &mut Ui, item: &mut Self::Item);

}
