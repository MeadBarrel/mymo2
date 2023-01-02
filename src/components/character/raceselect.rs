// 243-474-688-81

use crate::id::SuffixedId;
use eframe::egui::{self, Ui};
use crate::components::PropComponent;
use mymo::model::{Character, Parent};
use crate::widgets::RaceButton;

#[derive(Debug)]
pub(super) struct RaceSelectButton {
    id: SuffixedId,
    parent: Parent,
}

impl RaceSelectButton {
    pub fn new(id: SuffixedId, parent: Parent) -> Self {
        Self {
            id, parent,
        }
    }
}

impl PropComponent for RaceSelectButton {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut Ui, item: &mut Self::Item) {
        use mymo::model::avail_parents;

        let popup_id = self.id.derive("_popup").id();


        let response = ui.add(
            RaceButton::new(item.parents[self.parent])
        );

        if response.clicked() {
            ui.memory().open_popup(popup_id)
        }

        let available_races = avail_parents(item.clade, self.parent);
        
        egui::popup_below_widget(ui, popup_id, &response, |ui| {
            for race in available_races {
                let response = ui.add(
                    RaceButton::new(race)
                );
                if response.clicked() {
                    item.parents[self.parent] = race;
                }
            }
        });
    }
}
