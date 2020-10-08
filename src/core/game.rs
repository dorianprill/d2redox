use game::entity::player::Player;
use game::entity::npc::Npc;
use game::object::item::Item;
use game::object::world_object::WorldObject;
use game::areas::ActType;
use game::coordinate::Coordinate;

pub enum GameType {
    OpenBattleNet = 1,
    TCPIP         = 2,
    SinglePlayer  = 3
}

#[allow(dead_code)]
pub enum Difficulty {
    Normal     = 0,
    Nightmare  = 1,
    Hell       = 2
}

pub enum Locale {
    enUS    = 0,
    esES    = 1,
    deDE    = 2,
    frFR    = 3, 
    ptPT    = 4,
    itIT    = 5, 
    ja      = 6,
    ko      = 7,
    si      = 8,
    zhCN    = 9,
    pl      = 10, 
    ru      = 11, 
    enGB    = 12 
}


pub enum CharacterClass {
    Amazon      = 0,
    Sorceress   = 1,
    Necromancer = 2,
    Paladin     = 3,
    Barbarian   = 4,
    Druid       = 5,
    Assassin    = 6
}



#[allow(dead_code)]
pub struct Game {
    // skills: Vec<Skills>;
    // item_skills: Vec<ItemSkills>
    myself:         &Player,
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
