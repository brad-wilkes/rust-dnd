use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterClass {
    pub id: i32,
    pub name: String, // e.g. "fighter"
    pub hit_die: i16, // e.g., 8 => d8
    pub primary_abilities: Vec<String>,
    pub saving_throws: Vec<String>,
    pub armor_proficiencies: Vec<String>,
    pub weapon_proficiencies: Vec<String>,
    pub tool_proficiencies: Vec<String>,
    pub spellcasting: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: Option<i32>,
    pub name: String,
    pub class_id: i32,
    pub race: String,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignMembership {
    pub campaign_id: i32,
    pub character_id: i32,
}