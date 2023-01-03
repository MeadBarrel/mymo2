mod stats;
mod skills;

use mymo::model::Character;

use crate::id::SuffixedId;
use crate::containers::AutoMargin;

use super::PropComponent;

use eframe::egui::{self, Ui};

#[derive(Debug)]
pub struct CharacterEditor {
    id: SuffixedId,
    tab: Tab,
    stats: stats::CharacterStatsEditor,
    skills: skills::CharacterSkillsEditor,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    #[default]
    Stats,
    Skills,
}

impl Tab {
    pub fn name(self) -> String {
        match self {
            Tab::Stats => "Stats",
            Tab::Skills => "Skills"
        }.into()
    }
}

impl CharacterEditor {
    pub fn new(id: SuffixedId) -> Self {
        Self {
            tab: Default::default(),
            stats: stats::CharacterStatsEditor::new(id.derive("stats")),
            skills: skills::CharacterSkillsEditor::new(id.derive("skills")),
            id,
        }
    }

    fn tab_button(&mut self, ui: &mut egui::Ui, tab: Tab) -> egui::Response {
        let tab_text_style = egui::TextStyle::Name("Huge".into());
        let mut tab_richtext = egui::RichText::new(tab.name().to_uppercase()).text_style(tab_text_style);

        if self.tab == tab {
            tab_richtext = tab_richtext.strong();
            tab_richtext = tab_richtext.underline();
        };
        let tab_label = egui::Label::new(tab_richtext)
            .sense(egui::Sense::click());
        


        ui.add_space(10.);

        let response = ui.add(tab_label)
            .on_hover_cursor(egui::CursorIcon::PointingHand);

        if response.clicked() {
            println!("CLICKED {:?}", tab);
            self.tab = tab;
        }

        response
    }
}

impl PropComponent for CharacterEditor {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Self::Item) {
        egui::TopBottomPanel::top(self.id.derive("top"))
            .show_inside(ui, |ui| {     
                AutoMargin::new(self.id.derive("tabs_margin")).horizontal_centered().vertical_centered()
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            self.tab_button(ui, Tab::Stats);
                            self.tab_button(ui, Tab::Skills)
                        }).response
                    });
                
                // egui::Frame::default().fill(egui::Color32::from_additive_luminance(25)).show(ui, |ui| {
                //     let layout = egui::Layout::left_to_right(egui::Align::Center)
                //         .with_main_align(egui::Align::Center);
                //     ui.with_layout(layout, |ui| {
                //         self.tab_button(ui, Tab::Stats);
                //         self.tab_button(ui, Tab::Skills);
                //     })
                // })
                // ui.horizontal(|ui| {
                //     //ui.button(RichText::new("Stats").text_style(egui::TextStyle::Name("Huge".into())))
                //     self.tab_button(ui, Tab::Stats);
                //     //self.tab_button(ui, Tab::Skills);
                // })    
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.tab {
                Tab::Stats => self.stats.add(frame, ui, item),
                Tab::Skills => self.skills.add(frame, ui, item),
            }
        });
    }
}