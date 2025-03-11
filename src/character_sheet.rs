// character_sheet.rs
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Default, Serialize, Deserialize)]
pub struct CharacterSheet {
    pub name: String,
    pub class: String,
    pub race: String,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl CharacterSheet {
    pub fn new() -> Self {
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

    pub fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load_from_file(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}