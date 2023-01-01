mod propwrapper;
mod character;

pub use propwrapper::PropWrapper;
pub use character::CharacterEditor;

use eframe::egui::Ui;

pub trait Component {
    #[allow(unused)]
    fn add(&mut self, ui: &mut Ui);
}

pub trait PropComponent {
    type Item;

    #[allow(unused)]
    fn add(&mut self, ui: &mut Ui, item: &mut Self::Item);

}
