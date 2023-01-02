use eframe::egui::{Widget, Response};
use mymo::model::Race;

use super::*;

pub struct RaceButton {
    race: Race,
}

impl RaceButton {
    pub fn new(race: Race) -> Self {
        Self {
            race
        }
    }
}

impl Widget for RaceButton {
    fn ui(self, ui: &mut eframe::egui::Ui) -> Response {
        let image_name = race_image_name(self.race);
        ui.simple_image_button(&image_name)
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
        Risar => "race.thursar.png",
    }.to_string()
}