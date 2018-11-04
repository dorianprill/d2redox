use game::entity::*;
use game::object::*;
use game::areas::*;
use game::map::*;

const gs_port: u16 = 4000;


pub enum Difficulty {
    Normal,
    Nightmare,
    Hell
}

pub enum CharacterClass {
    Amazon,
    Sorceress,
    Necromancer,
    Paladin,
    Barbarian,
    Druid,
    Assassin
}

pub struct Game {
    // skills: Vec<Skills>;
    // item_skills: Vec<ItemSkills>
    //myself:         Player;
    // stash:       Container;
    // cube:        Container;
    // belt:        Belt;

    players:        Vec<entities::Player>,
    npcs:           Vec<entities::Npc>,
    items:          Vec<objects::Item>,
    objects:        Vec<objects::WorldObject>,
    current_act:    Areas::ActType,
}

impl Game {
    pub fn add_player(&self, id: u32) -> bool {
        return false
    }
    pub fn remove_player(&self, id: u32) -> bool {
        return false
    }
}
