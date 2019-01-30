use game::entity::player::Player;
use game::entity::npc::Npc;
use game::object::item::Item;
use game::object::world_object::WorldObject;
use game::areas::ActType;
use game::coordinate::Coordinate;
//use game::map::*;

#[allow(dead_code)]
pub enum Difficulty {
    Normal,
    Nightmare,
    Hell
}

#[allow(dead_code)]
pub enum CharacterClass {
    Amazon,
    Sorceress,
    Necromancer,
    Paladin,
    Barbarian,
    Druid,
    Assassin
}

#[allow(dead_code)]
pub struct Game {
    // skills: Vec<Skills>;
    // item_skills: Vec<ItemSkills>
    myself:         Player,
    // stash:       Container;
    // cube:        Container;
    // belt:        Belt;

    players:        Vec<Player>,
    npcs:           Vec<Npc>,
    items:          Vec<Item>,
    objects:        Vec<WorldObject>,
    current_act:    ActType,
}

impl Game {
    pub fn add_player(&self, id: u32) -> bool {
        return false
    }
    pub fn remove_player(&self, id: u32) -> bool {
        return false
    }
}
