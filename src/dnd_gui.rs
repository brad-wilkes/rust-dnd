use eframe::{egui, epi};
use crate::character_sheet::CharacterSheet;
use crate::dice::{DieType, roll_die};

pub struct DndGui {
    pub character: CharacterSheet,
    pub status: String,
    pub class_list: Vec<crate::models::CharacterClass>,
    pub selected_die: usize,
    pub last_roll: Option<u32>
}

impl Default for DndGui {
    fn default() -> Self {
        let class_list = crate::repositories::get_all_classes().unwrap_or_default();
        let selected_class = 0;
        let mut character = CharacterSheet::new();
        if let Some(first_class) = class_list.get(0) {
            character.class = first_class.name.clone();
        }
        Self {
            character,
            status: String::new(),
            class_list,
            selected_class,
            last_roll: None,
            selected_die: 0,
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

            ui.separator();
            ui.heading("Dice Roller");

            let die_types = DieType::all();
            let die_names: Vec<&str> = die_types.iter().map(|d| d.as_str()).collect();
            egui::ComboBox::from_label("Die")
                .selected_text(die_names[self.selected_die])
                .show_ui(ui, |ui| {
                    for (i, name) in die_names.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_die, i, *name);
                    }
                });

            if ui.button("Roll").clicked() {
                let die = die_types[self.selected_die].clone();
                self.last_roll = Some(roll_die(die));
            }

            if let Some(roll) = self.last_roll {
                ui.label(format!("Result: {}", roll));
            }
        });
    }
    fn name(&self) -> &str {
        "D&D Character Sheet"
    }
}