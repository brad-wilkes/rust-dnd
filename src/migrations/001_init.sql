CREATE TABLE IF NOT EXISTS character_classes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    hit_die SMALLINT NOT NULL,
    primary_abilities TEXT[] NOT NULL,
    armor_proficiencies TEXT[] NOT NULL,
    weapon_proficiencies TEXT[] NOT NULL,
    saving_throws TEXT[] NOT NULL,
    tool_proficiencies TEXT[] NOT NULL,
    spellcasting BOOLEAN NOT NULL DEFAULT FALSE
);

create table if not exists CAMPAIGNS (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT ''
);


CREATE TABLE IF NOT EXISTS characters (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    class_id INTEGER NOT NULL REFERENCES character_classes(id) ON DELETE RESTRICT,
    race TEXT NOT NULL,
    strength INTEGER NOT NULL,
    dexterity INTEGER NOT NULL,
    constitution INTEGER NOT NULL,
    intelligence INTEGER NOT NULL,
    wisdom INTEGER NOT NULL,
    charisma INTEGER NOT NULL
    );

CREATE TABLE IF NOT EXISTS campaign_memberships (
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    PRIMARY KEY (campaign_id, character_id)
    );

INSERT INTO character_classes (name, hit_die, primary_abilities, saving_throws, armor_proficiencies, weapon_proficiencies, tool_proficiencies, spellcasting)
VALUES
    ('fighter', 10, ARRAY['strength','constitution'], ARRAY['strength','constitution'], ARRAY['all armor','shields'], ARRAY['simple','martial'], ARRAY[]::text[], FALSE),
    ('warlock', 8, ARRAY['charisma','constitution'], ARRAY['wisdom','charisma'], ARRAY['light armor'], ARRAY['simple'], ARRAY[]::text[], TRUE),
    ('priest', 8, ARRAY['wisdom','constitution'], ARRAY['wisdom','charisma'], ARRAY['light armor','medium armor','shields'], ARRAY['simple'], ARRAY[]::text[], TRUE)
    ON CONFLICT DO NOTHING;
