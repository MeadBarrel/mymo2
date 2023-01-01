use eframe::egui::{TextureHandle, Widget, Response};
use mymo::model::Race;
use std::collections::HashMap;

use super::*;

pub struct RaceButton<'a> {
    images: &'a HashMap<String, TextureHandle>,
    race: Race,
}

impl<'a> RaceButton<'a> {
    pub fn new(images: &'a HashMap<String, TextureHandle>, race: Race) -> Self {
        Self {
            images, race
        }
    }
}

impl<'a> Widget for RaceButton<'a> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> Response {
        let image_name = race_image_name(self.race);
        ui.simple_image_button(self.images, &image_name)
    }
}

fn race_image_name(race: Race) -> String {
    use Race::*;
    match race {
        Tindremen => "race.tindremen.png",
        Sidoian => "race.sidoian.png",
        Sarducaan => "race.sarducaan.png",
        Khurite => "race.khurite.png",
        Kallard => "race.kallard.png",
        Veela => "race.veela.png",
        Sheevra => "race.sheevra.png",
        Blainn => "race.blainn.png",
        Huergar => "race.huergar.png",
        Thursar => "race.thursar.png",
    }.to_string()
}