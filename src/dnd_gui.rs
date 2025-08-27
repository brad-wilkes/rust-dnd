use eframe::{egui, epi};
use crate::character_sheet::CharacterSheet;

pub struct DndGui {
    pub character: CharacterSheet,
    pub status: String,
}

impl Default for DndGui {
    fn default() -> Self {
        Self {
            character: CharacterSheet::new(),
            status: String::new(),
        }
    }
}

impl epi::App for DndGui {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("D&D Character Sheet");

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.character.name);
            });

            ui.horizontal(|ui| {
                ui.label("Class:");
                ui.text_edit_singleline(&mut self.character.class);
            });

            ui.horizontal(|ui| {
                ui.label("Race:");
                ui.text_edit_singleline(&mut self.character.race);
            });

            ui.horizontal(|ui| {
                ui.label("Strength:");
                ui.add(egui::DragValue::new(&mut self.character.strength));
            });

            ui.horizontal(|ui| {
                ui.label("Dexterity:");
                ui.add(egui::DragValue::new(&mut self.character.dexterity));
            });

            ui.horizontal(|ui| {
                ui.label("Constitution:");
                ui.add(egui::DragValue::new(&mut self.character.constitution));
            });

            ui.horizontal(|ui| {
                ui.label("Intelligence:");
                ui.add(egui::DragValue::new(&mut self.character.intelligence));
            });

            ui.horizontal(|ui| {
                ui.label("Wisdom:");
                ui.add(egui::DragValue::new(&mut self.character.wisdom));
            });

            ui.horizontal(|ui| {
                ui.label("Charisma:");
                ui.add(egui::DragValue::new(&mut self.character.charisma));
            });

            ui.separator();

            if ui.button("Save").clicked() {
                match self.character.save_to_db() {
                    Ok(saved) => {
                        self.status = format!("Saved character '{}' (class_id={})", saved.name, saved.class_id);
                    }
                    Err(err) => {
                        self.status = format!("Error saving character: {}", err);
                    }
                }
            }

            if !self.status.is_empty() {
                ui.label(&self.status);
            }
        });
    }

    fn name(&self) -> &str {
        "D&D Character Sheet"
    }
}