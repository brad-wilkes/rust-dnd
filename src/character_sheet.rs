// character_sheet.rs
use serde::{Serialize, Deserialize};

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

    pub fn save_to_db(&self) -> Result<crate::models::Character, crate::repositories::RepoError> {
        let cls = crate::repositories::get_class_by_name(&self.class)?;
        let ch = crate::models::Character {
            id: None,
            name: self.name.clone(),
            class_id: cls.id,
            race: self.race.clone(),
            strength: self.strength,
            dexterity: self.dexterity,
            constitution: self.constitution,
            intelligence: self.intelligence,
            wisdom: self.wisdom,
            charisma: self.charisma,
        };
        crate::repositories::create_character(ch)
    }

    /// Load a CharacterSheet by character ID from the DB.
    pub fn load_from_db(id: i32) -> Result<Self, crate::repositories::RepoError> {
        let ch = crate::repositories::get_character(id)?;
        let cls = crate::repositories::get_class_by_id(ch.class_id)?;
        Ok(Self {
            name: ch.name,
            class: cls.name,
            race: ch.race,
            strength: ch.strength,
            dexterity: ch.dexterity,
            constitution: ch.constitution,
            intelligence: ch.intelligence,
            wisdom: ch.wisdom,
            charisma: ch.charisma,
        })
    }
}