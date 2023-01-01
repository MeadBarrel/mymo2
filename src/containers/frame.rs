use eframe::{egui::{InnerResponse, Color32, Frame, Ui}, epaint::Shadow};
use crate::storage::COLORS;

pub fn mymo_frame_defaults() -> Frame {
    Frame::none()
        .fill(COLORS.get().frame_color_std)
        .outer_margin(10.)
        .inner_margin(20.)
        .shadow(Shadow::small_light())
        .rounding(4.0)
}

pub fn frame<R>(ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    mymo_frame_defaults()
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            content(ui)
        })
}
