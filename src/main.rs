mod character_sheet;

use eframe::{egui, epi};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Default, Serialize, Deserialize)]
struct CharacterSheet {
    name: String,
    class: String,
    race: String,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

impl CharacterSheet {
    fn new() -> Self {
        Self {
            name: "Character Name".to_string(),
            class: "Class".to_string(),
            race: "Race".to_string(),
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
    fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(DndCharSheet::default()),
        options,
    );
}

struct DndCharSheet {
    character: CharacterSheet,
}

impl Default for DndCharSheet {
    fn default() -> Self {
        Self {
            character: CharacterSheet::new(),
        }
    }
}

impl epi::App for DndCharSheet {
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
            if ui.button("Save").clicked() {
                self.character.save_to_file("character_sheet.json");
            }
        });
    }

    fn name(&self) -> &str {
        "D&D Character Sheet"
    }
}