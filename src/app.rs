use eframe::egui::*;
use crate::components::{AppComponent, CharacterEditor, PropWrapper};
use std::rc::Rc;
use std::cell::Cell;


#[derive(Default)]
pub struct App {
    character: Rc<Cell<mymo::model::Character>>,
}


impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
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

        // ctx.set_style(
        //     Style::n
        // )

        CentralPanel::default().show(ctx, |ui| {
            PropWrapper::new(
                CharacterEditor::default(), self.character.clone()
            ).add(self, ui);
        });
    }
}
