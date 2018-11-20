use game::entity::*;
use game::object::*;
use game::areas::*;
use game::coordinate::*;
use game::map::*;


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
    myself:         entity::Player,
    // stash:       Container;
    // cube:        Container;
    // belt:        Belt;

    players:        Vec<entity::Player>,
    npcs:           Vec<entity::Npc>,
    items:          Vec<object::Item>,
    objects:        Vec<object::WorldObject>,
    current_act:    areas::ActType,
}

impl Game {
    pub fn add_player(&self, id: u32) -> bool {
        return false
    }
    pub fn remove_player(&self, id: u32) -> bool {
        return false
    }
}
