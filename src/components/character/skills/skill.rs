use mymo::model::Character;
use eframe::egui;

use crate::{id::SuffixedId, components::PropComponent};

use super::branch::Branch;

pub struct SkillEditor<'a> {
    id: SuffixedId,
    show_description: bool,
    show_parents: bool,
    branch: &'a Branch,
}

impl<'a> SkillEditor<'a> {
    pub fn new(id: SuffixedId, branch: &'a Branch) -> Self {
        Self { id, branch, show_description: false, show_parents: false }
    }

    pub fn show_description(mut self, show: bool) -> Self {
        self.show_description = show;
        self
    }

    pub fn show_parents(mut self, show: bool) -> Self {
        self.show_parents = show;
        self
    }
}

impl<'a> PropComponent for SkillEditor<'a> {
    type Item = Character;

    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui, item: &mut Self::Item) {
        let value_mut = &mut 50;

        let mut collapsing_header = egui::collapsing_header::CollapsingState::load_with_default_open
            (ui.ctx(), self.id.derive("collapsing_header").id(), false);
        
        let header_res = ui.horizontal(|ui| {
            let sense = egui::Sense::click();
            let value_label_rich = egui::RichText::new(value_mut.to_string())
                .text_style(egui::TextStyle::Name("Large".into()));
            
            let value_label = egui::Label::new(value_label_rich);

            let skilltype_label_rich = match self.branch.skill.is_primary {
                true =>  egui::RichText::new("pri")
                    .color(egui::Color32::from_rgb_additive(128, 0, 0))
                    .text_style(egui::TextStyle::Small),
                false => egui::RichText::new("sec")
                    .color(egui::Color32::from_rgb_additive(0, 128, 0))
                    .text_style(egui::TextStyle::Small),
            };

            let skilltype_label = egui::Label::new(skilltype_label_rich);

            let label_rich = egui::RichText::new(&self.branch.skill.name)
                .text_style(egui::TextStyle::Name("Large".into()));
            let label = egui::Label::new(label_rich)
                .sense(sense);

            collapsing_header.show_toggle_button(ui, egui::collapsing_header::paint_default_icon);                

            ui.add_sized([30., 0.], skilltype_label);


            if ui.add(label).clicked() {
                collapsing_header.toggle(ui)
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.add(value_label)
            })

        });

        collapsing_header.show_body_indented(&header_res.response, ui, |ui| {
            if self.show_description {
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    if let Some(desc) = self.branch.skill.description.as_ref() {
                        let description_rich = egui::RichText::new(desc)
                            .text_style(egui::TextStyle::Small);
                        ui.label(description_rich);
                    }
                    if self.show_parents && !self.branch.skill.parents.is_empty() {
                        ui.horizontal(|ui| {
                            let parents_label_rich = egui::RichText::new("parents: ")
                                .color(egui::Color32::from_additive_luminance(128))
                                .text_style(egui::TextStyle::Small);                            
                            ui.label(parents_label_rich);
                            for parent in self.branch.skill.parents.iter() {
                                let parent_fmt = format!("{} ({})", parent.0, parent.1);
                                let parent_label_rich = egui::RichText::new(parent_fmt)
                                    .text_style(egui::TextStyle::Small);
                                ui.label(parent_label_rich);
                            };
                        });
                    }
                });
            }

            ui.scope(|ui| {
                ui.spacing_mut().slider_width = ui.available_width() * 0.9;
                ui.add(egui::Slider::new(value_mut, 0..=100));
            });

            ui.horizontal(|ui| {
                if ui.button("0").clicked() {
                    *value_mut = 0
                }
                if ui.button("~70").clicked() {
                    *value_mut = 70
                }
                if ui.button("~100").clicked() {
                    *value_mut = 100
                }
                if ui.button("100").clicked() {
                    *value_mut = 100
                }
            });
            
            ui.vertical(|ui| {
                self.branch.children.iter().for_each(|child| {
                    SkillEditor::new(
                        self.id.derive(&format!("child_skill_{}", child.skill.name)), 
                        child,
                    )
                    .show_description(self.show_description)
                    .show_parents(self.show_parents)
                    .add(frame, ui, item)
                });    
            })
                
        });


    }

}