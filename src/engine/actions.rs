//use std::mem::transmute;

//use game::coordinate::*;
//use game::entity::player::Player;

#[repr(u8)]
enum GamePacketAction {
    Walk                    = 0x01,
    Run                     = 0x03,
    CastOnCoord             = 0x0c,
    CastOnObject            = 0x0d,
    UseItem                 = 0x20,
    StackItem               = 0x21,
    UseBeltItem             = 0x26,
    InsertSocketItem        = 0x28,
    ItemToCube              = 0x2A,
    NPCDialogueInit         = 0x2F,
    NPCDialogueCancel       = 0x30,
    NPCBuy                  = 0x32,
    NPCSell                 = 0x33,
    SwitchSkill             = 0x3c,
    UseWaypoint             = 0x49,
    RequestReassignment     = 0x4b,
    MakeEntityMove          = 0x59,
    Party                   = 0x5E,
    Relocate                = 0x5f,
    MercPotion              = 0x61,
    Ping                    = 0x6D,
}

fn u32_to_byte_array(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >>  8) & 0xff) as u8;
    let b4 : u8 =        (x  & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn u16_to_byte_array(x:u32) -> [u8;2] {
    let b1 : u8 = ((x >>  8) & 0xff) as u8;
    let b2 : u8 =        (x  & 0xff) as u8;
    return [b1, b2]
}

//
// fn build_packet(command: u8, args: &[u8]) -> &[u8] {
//     let packet: Vec<u8> = Vec::new();
//     let boxed:  Box<[u8]> = packet.into_boxed_slice();
//     boxed.push(command);
//     boxed.extend_from_slice(args);
//     return boxed;
// }
//
// pub fn walk<'a>(coords: Coordinate) -> &'a [u8] {
//     return build_packet(Action::Walk,
//                         u16_to_byte_array(coords.x),
//                         u16_to_byte_array(coords.y));
// }
//
// pub fn run<'a>(coords: Coordinate) -> &'a [u8] {
//     return build_packet(Action::Run,
//                         u16_to_byte_array(coords.x),
//                         u16_to_byte_array(coords.y));
// }
//
// pub fn relocate<'a>(coords: Coordinate) -> &'a [u8] {
//     return build_packet(Action::Relocate,
//                         u16_to_byte_array(coords.x),
//                         u16_to_byte_array(coords.y));
// }
//
// pub fn cast_on_coord<'a>(coords: Coordinate) -> &'a [u8] {
//     return build_packet(Action::CastOnCoord,
//                         u16_to_byte_array(coords.x),
//                         u16_to_byte_array(coords.y));
// }
//
// pub fn cast_on_player<'a>(player: Player) -> &'a [u8] {
//     return build_packet(Action::CastOnCoord,
//                         u16_to_byte_array(player.entity.location.x),
//                         u16_to_byte_array(player.entity.location.y));
// }

// pub fn cast_on_object(id: u32) -> &[u8] {
//     return build_packet(Action::CastOnObject,
//                         Connections.GenericDispatcher.one,
//                         u32_to_byte_array(id));
// }

// pub fn request_reassignment(id: u32) -> &[u8] {
//     return build_packet(Action::RequestReassignment,
//                         Connections.GenericDispatcher.nulls,
//                         u32_to_byte_array(id));
// }

// pub fn terminate_entity_dialogue(id: u32) -> &[u8] {
//     return build_packet(Action::TerminateEntityDialogue,
//                         Connections.GenericDispatcher.one,
//                         u32_to_byte_array(id));
// }

// pub fn make_entity_move(id: u32, coords: Coordinate) -> &[u8] {
//     return build_packet(Action::MakeEntityMove,
//                         Connections.GenericDispatcher.one,
//                         u32_to_byte_array(id),
//                         u32_to_byte_array(coords.x),
//                         Connections.GenericDispatcher.zero,
//                         Connections.GenericDispatcher.zero,
//                         u32_to_byte_array(coords.y),
//                         Connections.GenericDispatcher.zero,
//                         Connections.GenericDispatcher.zero);
// }

// pub fn switch_skill<'a>(skill: u32) -> &'a [u8] {
//     let temp: [u8] = [0xFF, 0xFF, 0xFF, 0xFF];
//     //little endian on linux and win but NOT mac32!
//     //let bytes: [u8; 4] = unsafe { transmute(skill.to_le()) };
//     return build_packet(Action::SwitchSkill, u32_to_byte_array(skill), temp);
// }

// pub fn drink_potion(id: u32) -> &[u8] {
//     return build_packet(Action::DrinkPotion,
//                         u32_to_byte_array(id),
//                         Connections.GenericDispatcher.nulls,
//                         Connections.GenericDispatcher.nulls);
// }
