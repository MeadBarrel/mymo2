use crate::widgets::RaceButton;
use mymo::model::Character;
use mymo::model::Attribute;
use mymo::strum::IntoEnumIterator;
use mymo::model::Parent;
use crate::id::SuffixedId;
use super::PropComponent;
use eframe::{egui::
    {self, Layout, Ui, InnerResponse}, 
    epaint::{Color32, Shadow}, 
    emath::Align
};

#[derive(Debug)]
pub struct CharacterEditor {
    id: SuffixedId,
    parent_buttons: Vec<RaceSelectButton>,
}

impl CharacterEditor {
    pub fn new(id: SuffixedId) -> Self {
        let parent_buttons = Parent::iter().enumerate().map(|(i, parent)| {
            RaceSelectButton::new(id.derive(&format!("select_parent_{i}")), parent)
        }).collect();
        Self {
            id,
            parent_buttons,
        }
    }
}

impl PropComponent for CharacterEditor {
    type Item = Character;

    fn add(&mut self, ui: &mut eframe::egui::Ui, item: &mut Self::Item) {
        let side_panel_width = ui.available_width() * 0.35;
        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            egui::SidePanel::left(self.id.derive("side_panel"))
                .resizable(false)
                .exact_width(side_panel_width)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.);
                        frame(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.heading("Race:");
                                    self.parent_buttons.iter_mut().for_each(|parent_button| {
                                        ui.add_space(10.);
                                        parent_button.add(ui, item)
                                    });
                                })
    
                            });
                            
                        })
                    });
                });

        
            egui::SidePanel::right(self.id.derive("left_panel"))
                .exact_width(side_panel_width)
                .resizable(false)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                   
                });

            
            egui::CentralPanel::default()
                .show_inside(ui, |ui| {
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = ui.available_width() * 0.9;
                        Attribute::iter().for_each(|attribute| {
                            frame(ui, |ui| {
                                attribute_frame(ui, item, attribute)
                            });                
                        })    
                    })
                });
        });

    }
}

fn attribute_frame(ui: &mut Ui, character: &mut Character, attribute: Attribute) -> InnerResponse<()> {
    let attribute_cap = character.attribute_cap(attribute);
    ui.vertical(|ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add_space(5.);
                ui.heading(&attribute.name());
            });
        
            ui.with_layout(Layout::top_down(Align::Max), |ui| {
                    ui.label(attribute_cap.to_string());
                    ui.label(character.attributes[attribute].to_string());
            })
        });
        ui.add_space(5.);
        ui.add(
            egui::Slider::new(&mut character.attributes[attribute], 10..=attribute_cap)
                .show_value(false)
        );
    })    
}

fn frame<R>(ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    egui::Frame::none()
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


#[derive(Debug)]
struct RaceSelectButton {
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

    fn add(&mut self, ui: &mut Ui, item: &mut Self::Item) {
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
