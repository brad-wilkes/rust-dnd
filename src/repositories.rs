use crate::models;
use crate::db::{pool, PgPooledConn, DbInitError};
use crate::models::{Campaign, CampaignMembership, Character, CharacterClass};
use postgres::Row;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("db error: {0}")]
    Db(#[from] r2d2::Error),
    #[error("pg error: {0}")]
    Pg(#[from] postgres::Error),
    #[error("db init error: {0}")]
    Init(#[from] DbInitError),
    #[error("not found")]
    NotFound,
}



fn map_character(row: Row) -> Character {
    Character {
        id: Some(row.get::<_, i32>("id")),
        name: row.get("name"),
        class_id: row.get("class_id"),
        race: row.get("race"),
        strength: row.get("strength"),
        dexterity: row.get("dexterity"),
        constitution: row.get("constitution"),
        intelligence: row.get("intelligence"),
        wisdom: row.get("wisdom"),
        charisma: row.get("charisma"),
    }
}

fn map_class(row: Row) -> CharacterClass {
    CharacterClass {
        id: row.get("id"),
        name: row.get("name"),
        hit_die: row.get("hit_die"),
        primary_abilities: row.get("primary_abilities"),
        saving_throws: row.get("saving_throws"),
        armor_proficiencies: row.get("armor_proficiencies"),
        weapon_proficiencies: row.get("weapon_proficiencies"),
        tool_proficiencies: row.get("tool_proficiencies"),
        spellcasting: row.get("spellcasting"),
    }
}

fn map_campaign(row: Row) -> Campaign {
    Campaign {
        id: Some(row.get::<_, i32>("id")),
        name: row.get("name"),
        description: row.get("description"),
    }
}

fn conn() -> Result<PgPooledConn, RepoError> {
    Ok(pool()?.get()?)
}

// CharacterClass
// pub fn list_classes() -> Result<Vec<CharacterClass>, RepoError> {
//     let mut c = conn()?;
//     let rows = c.query("SELECT * FROM character_classes ORDER BY name", &[])?;
//     Ok(rows.into_iter().map(map_class).collect())
// }

pub fn get_all_classes() -> Result<Vec<models::CharacterClass>, RepoError> {
    let pool = crate::db::pool()?;
    let mut conn = pool.get()?;
    let rows = conn.query(
        "SELECT id, name, hit_die, primary_abilities, saving_throws, armor_proficiencies, weapon_proficiencies, tool_proficiencies, spellcasting FROM character_classes",
        &[]
    )?;
    Ok(rows.into_iter().map(|row| {
        crate::models::CharacterClass {
            id: row.get(0),
            name: row.get(1),
            hit_die: row.get(2),
            primary_abilities: row.get(3),
            saving_throws: row.get(4),
            armor_proficiencies: row.get(5),
            weapon_proficiencies: row.get(6),
            tool_proficiencies: row.get(7),
            spellcasting: row.get(8),
        }
    }).collect())
}

    pub fn get_class_by_name(name: &str) -> Result<CharacterClass, RepoError> {
    let mut c = conn()?;
    let row = c
        .query_opt("SELECT * FROM character_classes WHERE name = $1", &[&name])?
        .ok_or(RepoError::NotFound)?;
    Ok(map_class(row))
}

pub fn get_class_by_id(id: i32) -> Result<CharacterClass, RepoError> {
    let mut c = conn()?;
    let row = c
        .query_opt("SELECT * FROM character_classes WHERE id = $1", &[&id])?
        .ok_or(RepoError::NotFound)?;
    Ok(map_class(row))
}

// Characters
pub fn create_character(mut ch: Character) -> Result<Character, RepoError> {
    let mut c = conn()?;
    let row = c.query_one(
        "INSERT INTO characters
         (name, class_id, race, strength, dexterity, constitution, intelligence, wisdom, charisma)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
         RETURNING *",
        &[
            &ch.name,
            &ch.class_id,
            &ch.race,
            &ch.strength,
            &ch.dexterity,
            &ch.constitution,
            &ch.intelligence,
            &ch.wisdom,
            &ch.charisma,
        ],
    )?;
    ch = map_character(row);
    Ok(ch)
}

pub fn update_character(ch: &Character) -> Result<Character, RepoError> {
    let id = ch.id.ok_or(RepoError::NotFound)?;
    let mut c = conn()?;
    let row = c.query_one(
        "UPDATE characters
         SET name=$1, class_id=$2, race=$3, strength=$4, dexterity=$5,
             constitution=$6, intelligence=$7, wisdom=$8, charisma=$9
         WHERE id=$10
         RETURNING *",
        &[
            &ch.name,
            &ch.class_id,
            &ch.race,
            &ch.strength,
            &ch.dexterity,
            &ch.constitution,
            &ch.intelligence,
            &ch.wisdom,
            &ch.charisma,
            &id,
        ],
    )?;
    Ok(map_character(row))
}

pub fn get_character(id: i32) -> Result<Character, RepoError> {
    let mut c = conn()?;
    let row = c
        .query_opt("SELECT * FROM characters WHERE id = $1", &[&id])?
        .ok_or(RepoError::NotFound)?;
    Ok(map_character(row))
}

pub fn list_characters() -> Result<Vec<Character>, RepoError> {
    let mut c = conn()?;
    let rows = c.query("SELECT * FROM characters ORDER BY id DESC", &[])?;
    Ok(rows.into_iter().map(map_character).collect())
}

pub fn delete_character(id: i32) -> Result<(), RepoError> {
    let mut c = conn()?;
    c.execute("DELETE FROM characters WHERE id=$1", &[&id])?;
    Ok(())
}

// Campaigns
pub fn create_campaign(mut cp: Campaign) -> Result<Campaign, RepoError> {
    let mut c = conn()?;
    let row = c.query_one(
        "INSERT INTO campaigns (name, description) VALUES ($1,$2) RETURNING *",
        &[&cp.name, &cp.description],
    )?;
    cp = map_campaign(row);
    Ok(cp)
}

pub fn list_campaigns() -> Result<Vec<Campaign>, RepoError> {
    let mut c = conn()?;
    let rows = c.query("SELECT * FROM campaigns ORDER BY name", &[])?;
    Ok(rows.into_iter().map(map_campaign).collect())
}

pub fn add_character_to_campaign(cm: &CampaignMembership) -> Result<(), RepoError> {
    let mut c = conn()?;
    c.execute(
        "INSERT INTO campaign_memberships (campaign_id, character_id)
         VALUES ($1,$2) ON CONFLICT DO NOTHING",
        &[&cm.campaign_id, &cm.character_id],
    )?;
    Ok(())
}

pub fn list_campaign_members(campaign_id: i32) -> Result<Vec<Character>, RepoError> {
    let mut c = conn()?;
    let rows = c.query(
        "SELECT ch.*
         FROM campaign_memberships m
         JOIN characters ch ON ch.id = m.character_id
         WHERE m.campaign_id = $1
         ORDER BY ch.name",
        &[&campaign_id],
    )?;
    Ok(rows.into_iter().map(map_character).collect())
}