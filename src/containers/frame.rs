use eframe::{egui::{InnerResponse, Color32, Frame, Ui}, epaint::Shadow};

pub fn frame<R>(ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Frame::none()
        .fill(Color32::from_additive_luminance(25))
        .outer_margin(10.)
        .inner_margin(20.)
        .shadow(Shadow::small_light())
        .rounding(4.0)
        .show(ui, |ui| {     
            ui.set_width(ui.available_width());
            content(ui)
        })
}
