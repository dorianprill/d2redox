pub mod player;
pub mod npc;
pub mod world_entity;
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
