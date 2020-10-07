pub mod player;
pub mod npc;
//pub mod world_entity;
pub mod mercenary;


#[allow(dead_code)]
pub enum EntityType {
    Player      = 0x00,
    NPC         = 0x01, // NPC, Mercenary, Monster
    WorldEntity = 0x02, //Stash, Waypoint, Chests, Portals, others.
    Missiles    = 0x03,
    Items       = 0x04,
    Entrance    = 0x05
}

use game::coordinate::Coordinate;

pub type EntityId = u32;

#[allow(dead_code)]
pub struct Entity {
    initialized:    bool,
    id:             EntityId,
    location:       Coordinate
}

impl Entity {

    pub fn new(id: u32, x: u16, y: u16) -> Entity{
        Entity{initialized: true, id: id, location: Coordinate::new(x, y)}
    }

    pub fn initialized(&self) -> bool {
        return self.initialized
    }

    pub fn id(&self) -> u32 {
        return self.id
    }

    pub fn location(self) -> Coordinate {
        return self.location
    }
}

