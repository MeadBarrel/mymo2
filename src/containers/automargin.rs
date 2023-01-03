use eframe::egui::{self, Response};
use egui::style::Margin;

use crate::id::SuffixedId;

pub struct AutoMargin {
    id: SuffixedId,
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

impl AutoMargin {
    pub fn new(id: SuffixedId) -> Self {
        Self {
            id,
            left: false,
            right: false,
            top: false,
            bottom: false,
        }
    }

    pub fn left(mut self) -> Self {
        self.left = true;
        self
    }

    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    pub fn top(mut self) -> Self {
        self.top = true;
        self
    }

    pub fn bottom(mut self) -> Self {
        self.bottom = true;
        self
    }

    pub fn horizontal_centered(self) -> Self {
        self.left().right()
    }

    pub fn vertical_centered(self) -> Self {
        self.top().bottom()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> Response) {
        let known_rect: Option<egui::Rect> = {
            let mut data = ui.ctx().data();
            data.get_temp(self.id.id())
        };

        let available_size = ui.available_size();

        match known_rect {
            Some(rect) => {
                let left = match (self.left, self.right) {
                    (true, true) => (available_size.x - rect.right() + rect.left()) / 2.,
                    (true, false) => available_size.x - rect.right() + rect.left(),
                    _ => 0.
                };
                let top = match (self.top, self.bottom) {
                    (true, true) => (available_size.y - rect.bottom() + rect.top()) / 2.,
                    (true, false) => available_size.y - rect.bottom() - rect.top(),
                    _ => 0.
                };
                let frame = egui::Frame::default().outer_margin(
                    Margin { left, top, right: 0., bottom: 0. }
                );
                let new_rect = frame.show(ui, |ui| {
                    add_contents(ui).rect
                }).inner;
                ui.ctx().data().insert_temp(self.id.id(), new_rect);
            }
            None => {                
                let rect = add_contents(ui).rect;
                ui.ctx().data().insert_temp(self.id.id(), rect);
            }
        }
    }

    // pub fn show(&mut self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> Response) {
    //     let space = {
    //         let mut data = ui.ctx().data();
    //         data.get_temp::<(f32, f32, egui::Vec2)>(self.id.id())
    //     };
    //     let current_size = ui.available_size();
    //     match space {
    //         Some((width, height, old_size)) if old_size == current_size => {                
    //             let left = match (self.left, self.right) {
    //                 (true, true) => width/2.,
    //                 (true, false) => width,
    //                 _ => 0.,
    //             };
    //             let top = match (self.top, self.bottom) {
    //                 (true, true) => height/2.,
    //                 (true, false) => height,
    //                 _ => 0.
    //             };
    //             let frame = egui::Frame::default()
    //                 .outer_margin(Margin { left, top, ..Default::default() });
    //             frame.show(ui, add_contents);
    //         },
    //         None | Some(_) => {
    //             ui.
    //             let response = add_contents(ui);
    //             println!("{:?}", response.rect);
    //             let width = ui.available_width();
    //             let height = ui.available_height();
    //             println!("{:?}, {:?}", ui.available_size(), current_size);
    //             ui.ctx().data().insert_temp(self.id.id(), (width, height, current_size))    
    //         }
    //     }
        //data.insert_temp(self.id, value)
}
