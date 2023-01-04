use eframe::egui::*;
use crate::components::{Component, CharacterEditor, PropWrapper};
use crate::id::SuffixedId;
use std::rc::Rc;
use std::cell::Cell;

pub struct App {
    character: Rc<Cell<mymo::model::Character>>,
    character_editor: PropWrapper<mymo::model::Character, CharacterEditor>,
    id: SuffixedId,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        let id = SuffixedId::default();
        let character: Rc<Cell<mymo::model::Character>> = Default::default();
        Self {
            character_editor: PropWrapper::new(
                CharacterEditor::new(id.derive("character_editor")), 
                character.clone(),
            ),
            character,
            id,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {

        let mut style = Style::default();
        use TextStyle::*;
        use eframe::egui::FontFamily::Proportional;

        style.text_styles = [            
            (Heading, FontId::new(24.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(18.0, Proportional)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(14.0, Proportional)),
            (Name("Huge".into()), FontId::new(32.0, Proportional)),
            (Name("Large".into()), FontId::new(24.0, Proportional)),
            (Name("Tiny".into()), FontId::new(8.0, Proportional)),
        ].into();

        ctx.set_style(style);

        CentralPanel::default().show(ctx, |ui| {
            self.character_editor.add(frame, ui)
        });
    }
}
