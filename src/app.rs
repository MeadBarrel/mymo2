use eframe::egui::*;
use crate::components::{Component, CharacterEditor, PropWrapper};
use crate::id::SuffixedId;
use std::rc::Rc;
use std::cell::Cell;

#[derive(Default)]
pub struct App {
    character: Rc<Cell<mymo::model::Character>>,
    id: SuffixedId,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
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
            (Monospace, FontId::new(18.0, Proportional)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(14.0, Proportional)),
        ].into();

        ctx.set_style(style);

        CentralPanel::default().show(ctx, |ui| {
            PropWrapper::new(
                CharacterEditor::new(self.id.derive("character_editor")), self.character.clone()
            ).add(ui);
        });
    }
}
