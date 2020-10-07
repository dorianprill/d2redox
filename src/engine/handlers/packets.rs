// packets.rs
// this module contains the definitions of the client->server
// and server->client packets and the messages they contain.

use std::fmt;
//use std::convert::From;
//use engine::handlers::game_event::*;
use connection::d2gs::d2gs_packet::D2GSPacket;
//use game::entity::{EntityType, EntityId};
//use game::entity::player::PlayerItemSlot;
//use game::object::item::{ItemId, Item, ItemBuffer, ItemBufferCoord};

pub enum GamePacketId {
    ClientMessage,
    ServerPacketId
}

impl fmt::Display for GamePacketId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// D2 Game Server Message Identifiers
/// Naming shall follow bnetdocs.org naming unless shown to be inappropriate
/// Server->Client ClientMessage is determined by the first byte of a D2GSPacket's data
/// The specific arguments argument a packet takes is defined in the respective packet builders
#[repr(u8)]
pub enum ServerPacketId {
    
    CharToObj               = 0x10,
    ReportKill              = 0x11,

    SetGameObjectMode       = 0x14,
    PlayerReassign          = 0x15, // TODO this is used for self sent chat packets!?

    LifeManaUpdate1         = 0x18, //TODO: GS: 15   18 5d 00 16 80 21 40 0b c0 f1 b1 a8 f0 ff 07
    SmallGoldPickup         = 0x19,
    SetByteAttr             = 0x1D,
    SetWordAttr             = 0x1E,
    SetDWordAttr            = 0x1F,
    StateNotification       = 0x20,
    UpdatePlayerSkill       = 0x21,
    UpdatePlayerItemSkill   = 0x22,
    PlayerAssignSkill       = 0x23,

    NpcInteraction          = 0x27, // TODO: parse Merc Info, split in two classes / events: TownFolkInteract and MercInfo.
    UpdateQuestsInfo        = 0x28,
    GameQuestsInfo          = 0x29,
    TradeResult             = 0x2A,

    PlaySound               = 0x2C,
    UpdateContainerItem     = 0x3E,
    UseStackableItem        = 0x3F,
    PlayerClearCursor       = 0x42,
    Relator1                = 0x47,
    Relator2                = 0x48,


    // Other unit related actions
    UnitUseSkillOnTarget    = 0x4C,
    UnitUseSkill            = 0x4D,
    MercForHire             = 0x4E,

    QuestSpecial			= 0x50,	//TODO: length = 15
    WorldObject             = 0x51,
    PlayerQuestLog          = 0x52, //TODO: figure out state values...
    PartyRefresh            = 0x53,

    PlayerAssign            = 0x59, 
    PlayerInformation       = 0x5A,
    PlayerJoined            = 0x5B,
    StartGame               = 0x5C, // This packet is part of the logon sequence, not to be confused with the other 0x5C. This packet is originally received compressed, so the message ID will correspond with [Protocol Headers] D2GS compressed format. This message is received with the 'OK' that you can go ahead and enter the gaming environment.
    
    QuestItemState			= 0x5D,	// TODO: length = 6
    GameQuestAvailability   = 0x5E, // TODO
    //UnknownGame			  = 0x5F,	//TODO: Part of join data, after GameHandshake... 5f 01 00 00 00

    PortalInfo              = 0x60,

    OpenWaypoint            = 0x63,

    PlayerKillCount         = 0x65,

    NpcMoveStart            = 0x67,
    NpcMoveToTarget         = 0x68, // ?
    NpcStateUpdate          = 0x69, // ?
    NpcAction               = 0x6B,
    NpcAttack               = 0x6C, //MonsterAttack

    NpcMoveStop             = 0x6D,

    AboutPlayer             = 0x75,
    OverHeadClear           = 0x76,
    UpdateItemUI            = 0x77,
    TradeAccept             = 0x78,
    TradeGold               = 0x79,
    SummonAction            = 0x7A,
    AssignSkillHotkey       = 0x7B,
    UseSpecialItem          = 0x7C,	 //TODO: Only type 4 : Identify / portal tome / scroll is known
    //UnknownGame			  = 0x7E,	//TODO: Part of join data, after PlayerReassign... 7e 10 00 00 34
    PartyMemberUpdate       = 0x7F,

    MercUpdate              = 0x81,
    PortalUpdate            = 0x82,

    NpcWantsInteract        = 0x8A,
    PlayerPartyRelation     = 0x8B,
    PlayerRelation          = 0x8C,
    AssignPlayerToParty     = 0x8D,
    CorpseAssign            = 0x8E,
    Pong                    = 0x8F,
    PartyMemberPulse        = 0x90,

    //UNKNOWN				= 0x91, // Length = 26. At the start of map unload data... 91 00 93 00 ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff
    SkillsLog               = 0x94,
    PlayerLifeManaChange    = 0x95, // not sure why there are two types for hp/mp update (0x18 and 0x95)
    WalkVerify              = 0x96,
    //UNKNOWN				= 0x97,	// Length = 1. Part of join data, towards the end. Initialise / trigger something ?
    //UNKNOWN				= 0x9B,	// Merc related... res a rogue merc in act 3 : 9b ff ff 00 00 00 00
    ItemWorldAction         = 0x9C,
    ItemOwnedAction         = 0x9D,
    MercAttributeByte       = 0x9E,
    MercAttributeWord       = 0x9F,
    MercAttributeDWord      = 0xA0,

    DelayedState            = 0xA7,
    SetState                = 0xA8,
    EndState                = 0xA9,
    AddUnit                 = 0xAA,
    NpcHeal                 = 0xAB,
    NpcAssignment           = 0xAC,

    WardenScan              = 0xAE,
    NegotiateCompression    = 0xAF, // The compression mode is one of: 0x81 : custom compression mode, data follows (unused) 0x01 : compression enabled, use default compression 0x00 : no compression (unused). In practice the server ALWAYS uses compression.

    GameOver                = 0xB0,
    Invalid                 = 0xB1,
}

impl fmt::Display for ServerPacketId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match *self {
            ServerPacketId::CharToObj               => "CharToObj",
            ServerPacketId::ReportKill              => "ReportKill",

            ServerPacketId::SetGameObjectMode       => "SetGameObjectMode",
            ServerPacketId::PlayerReassign          => "PlayerReassign", 

            ServerPacketId::LifeManaUpdate1         => "LifeManaUpdate1",
            ServerPacketId::SmallGoldPickup         => "SmallGoldPickup",
            ServerPacketId::SetByteAttr             => "SetByteAttr",
            ServerPacketId::SetWordAttr             => "SetWordAttr",
            ServerPacketId::SetDWordAttr            => "SetDWordAttr",
            ServerPacketId::StateNotification       => "StateNotification",
            ServerPacketId::UpdatePlayerSkill       => "UpdatePlayerSkill",
            ServerPacketId::UpdatePlayerItemSkill   => "UpdatePlayerItemSkill",
            ServerPacketId::PlayerAssignSkill       => "PlayerAssignSkill",

            ServerPacketId::NpcInteraction          => "NpcInteraction", 
            ServerPacketId::UpdateQuestsInfo        => "UpdateQuestsInfo",
            ServerPacketId::GameQuestsInfo          => "GameQuestInfo",
            ServerPacketId::TradeResult             => "TradeResult",

            ServerPacketId::PlaySound               => "PlaySound",
            ServerPacketId::UpdateContainerItem     => "UpdateContainerItem",
            ServerPacketId::UseStackableItem        => "UseStackableItem",
            ServerPacketId::PlayerClearCursor       => "PlayerClearCursor",
            ServerPacketId::Relator1                => "Relator1",
            ServerPacketId::Relator2                => "Relator2",


            ServerPacketId::UnitUseSkillOnTarget    => "UnitUseSkillOnTarget",
            ServerPacketId::UnitUseSkill            => "UnitUseSkill",
            ServerPacketId::MercForHire             => "MercForHire",

            ServerPacketId::QuestSpecial			=> "QuestSpecial",	
            ServerPacketId::WorldObject             => "WorldObject",
            ServerPacketId::PlayerQuestLog          => "PlayerQuestLog", 
            ServerPacketId::PartyRefresh            => "PartyRefresh",

            ServerPacketId::PlayerAssign            => "PlayerAssign", 
            ServerPacketId::PlayerInformation       => "PlayerInformation",
            ServerPacketId::PlayerJoined            => "PlayerJoined",
            ServerPacketId::StartGame               => "StartGame", 
    
            ServerPacketId::QuestItemState			=> "QuestItemState",	
            ServerPacketId::GameQuestAvailability   => "GameQuestAvailability",             

            ServerPacketId::PortalInfo              => "PortalInfo",

            ServerPacketId::OpenWaypoint            => "OpenWaypoint",

            ServerPacketId::PlayerKillCount         => "PlayerKillCount",

            ServerPacketId::NpcMoveStart            => "NocMoveStart",
            ServerPacketId::NpcMoveToTarget         => "NpcMoveToTarget",
            ServerPacketId::NpcStateUpdate          => "NpcStateUpdate", 
            ServerPacketId::NpcAction               => "NpcAction",
            ServerPacketId::NpcAttack               => "NpcAttack",

            ServerPacketId::NpcMoveStop             => "NpcMoveStop",

            ServerPacketId::AboutPlayer             => "AboutPlayer",
            ServerPacketId::OverHeadClear           => "OverHeadClear",
            ServerPacketId::UpdateItemUI            => "UpdateItemUI",
            ServerPacketId::TradeAccept             => "TradeAccept",
            _ =>                                    "UnknownServerPacketId"
        };
        write!(f, "{}: {}", self, print)
    }
}
/// D2 Game Client Message Identifiers
/// Naming shall follow bnetdocs.org naming unless shown to be inappropriate
/// The specific arguments argument a packet takes is defined in the respective packet builders 
/// The enum discriminants contain the correct packet data types for the corresponding packet type
#[repr(u8)]
pub enum ClientMessage {
    // Walk to a specified (X,Y) map coordinate
    WalkToLocation{x: u16, y: u16}              = 0x01, 
    // Makes your character walk to the Entity specified in Entity ID
    WalkToEntity{entity_type: u32, entity_id: u32}          = 0x02, 
    // Run to a specified (X,Y) map coordinate
    RunToLocation{x: u16, y: u16}               = 0x03, 
    // Makes your character run to the Entity specified in Entity ID
    RunToEntity{entity_type: u32, entity_id: u32}           = 0x04, 
    // Uses Left skill on specified (X,Y) map coordinate
    LeftSkillOnLocation{x: u16, y: u16}         = 0x05, 
    // Uses your left skill on the Entity specefied in Entity ID
    LeftSkillOnEntity{entity_type: u32, entity_id: u32}     = 0x06, 
    // Uses your left skill on the Entity specefied in Entity ID, 
    // while holding the hotkey for standing still(shift).
    LeftSkillOnEntityEx{entity_type: u32, entity_id: u32}   = 0x07, 
    // Uses Left skill on specified (X,Y) map coordinate. 
    // This packet is sent repeatedly when the mouse button is held 
    // down after the initial packet has been sent. 
    LeftSkillOnLocationEx{x: u16, y: u16}       = 0x08, 
    // Uses your left skill on the Entity specified in Entity ID.
    // This packet is sent repeatedly when the mouse button is held 
    // down after the initial packet has been sent.
    LeftSkillOnEntityEx2{entity_type: u32, entity_id: u32}  = 0x09, 
    // Uses your left skill on the Entity specefied in Entity ID, while holding 
    // the hotkey for standing still(shift). This packet is sent repeatedly 
    // when the mouse button is held down after the initial packet has been sent. 
    LeftSkillOnEntityEx3{entity_type: u32, entity_id: u32}  = 0x0A, 
    //GameHandshake           = 0x0B, // TODO server message?
    // Uses the currently selected skill at the specified location
    RightSkillOnLocation{x: u16, y: u16}        = 0x0C, 
    // Uses your right skill on the Entity specefied in Entity ID
    RightSkillOnEntity{entity_type: u32, entity_id: u32}    = 0x0D, 
    // Uses your right skill on the Entity specefied in Entity ID, while holding the hotkey
    RightSkillOnEntityEx{entity_type: u32, entity_id: u32}  = 0x0E, 
    // Uses the currently selected skill at the specified location
    RightSkillOnLocationEx{x: u16, y: u16}      = 0x0F, 
    // Uses your right skill repeatedly on the Entity specified in Entity ID
    RightSkillOnEntityEx2{entity_type: u32, entity_id: u32} = 0x10, 
    // Uses your right skill on the Entity specefied in Entity ID, while holding the hotkey
    RightSkillOnEntityEx3{entity_type: u32, entity_id: u32} = 0x11, 
    // Interacts with the specified Entity. For players and npc's, it will send a request to interact. 
    // The interaction depends on the type of the unit or object. For others it will trigger the object, for example using a shrine, looting a corpse you have permission to loot, or opening and closing a door.
    InteractWithEntity{entity_type: u32, entity_id: u32}    = 0x13, 
    // This message is used when you'd like to put a message above a character's head.
    // Restrictions: 
    //   Total size of packet cannot be greater than 275 bytes. 
    //   Message cannot be greater than 255 bytes.
    // Public colors can be created by adding hex FF 63 and a character 30 to 3C
    OverheadMessage{unknown: u16, message: String, unused: u8, unknown2: u16} = 0x14, 
    //PlayerReassign          = 0x15, // TODO: server message?
    // Pick up a ground item to cursor buffer/inventory
    //PickupItem(UnitType: u32, UnitId: u32, ActionId: u32)      = 0x16, // TODO what unit types exist?
    // Drops the item in the player's cursor buffer to the ground
    DropItem{item_id: u32} = 0x17,
    // Moves item form the player's cursor buffer to an inventory space (should'nt it be name FromBuffer?)
    ItemToBuffer{item_id: u32 , x: u32, y: u32, item_buffer: u32} = 0x18, 
    // Pickup an item from the possible buffers below, moving it to the cursor buffer
    PickupBufferItem{item_id: u32}                = 0x19, 
    // Moves item from player's cursor buffer to body location
    ItemToBody{item_id: u32, player_item_slot: u32} = 0x1A, 
    // Moves item from body location to player's cursor buffer
    Swap2HandedItem{item_id: u32, player_item_slot: u32} = 0x1B, 
    // Pickup an item from a Body Location to you're cursor buffer
    PickupBodyItem{player_item_slot: u16}     = 0x1C, 
    // Swaps item in player's cursor buffer with item in the body location
    SwitchBodyItem{item_id: u32, player_item_slot: u32}  = 0x1D, 
    // Uses the specified item (such as a potion, town portal, etc)
    UseItem{item_id: u32, x: u32, y: u32}                   = 0x20, 
    // Stacks one item such as a key onto another item
    StackItem{item_id: u32, item_id_target: u32}            = 0x21,
    // DEPRECATED
    RemoveStackItem{item_id: u32}                           = 0x22, 
    // Moves an item into the player's belt
    ItemToBelt{item_id: u32, belt_slot: u32}                = 0x23, 
    // Moves the specified item from the belt to the player's cursor buffer
    RemoveBeltItem{item_id: u32}                            = 0x24, 
    // Replaces item in belt with item in player's cursor buffer
    SwitchBeltItem{item_id_cursor: u32, item_id: u32}       = 0x25, 
    // Uses the specified item in the player's belt.
    UseBeltItem{item_id: u32, unknown: u32, unknown2: u32}  = 0x26, 
    // Inserts the specified item into a socketed item
    InsertSocketItem{socketable_item: u32, item_id: u32}    = 0x28, 
    ScrollToTome            = 0x29, // Places a scroll into a tome of scrolls
    ItemToCube              = 0x2A, // Moves item from player's cursor buffer into Horadric cube
    
    UnselectObj             = 0x2D, // DEPRECATED
    NpcInit                 = 0x2F, // Initiate an NPC sesstion, sent following: C->S 0x13 It indicates that you are now interacting with an NPC, and a dialog window is opened. This is prior to any choices being made to talk or trade etc.
    NpcCancel               = 0x30, // Stops interacting with an NPC 

    NpcBuy                  = 0x32, // Buys an item from a Non Player Character
    NpcSell                 = 0x33, // Sell an item to a Non Player Character
    NpcTrade                = 0x38, // This packet's use is currently unconfirmed (bnetdocs)

    CharacterPhrase         = 0x3F, // All phrases sent to the server will be heard by all players in your vicinity

    Waypoint                = 0x49, // Requests to go to a waypoint if it was already activated

    // This message should be used for manipulating the trading window,
    // the Horadric Cube item window, and the Stash window.
    // see https://bnetdocs.org/packet/98/d2gs-trade
    Trade                   = 0x4F,
    DropGold			    = 0x50,	// Drops a pile of gold to the ground
    
    Party			        = 0x5E, // Possible Action IDs: 0x06 - Invite player to party with you 0x07 - Cancel invite to player 0x08 - Accept invite from player 0x09 - Leave party

    PortalInfo              = 0x60,
    PotionMercenary         = 0x61, // Takes the potion your cursor holds and gives it to the mercenary

    GameCreate              = 0x67, // Replaces 0x67 D2BS_GAMELOGON when creating a new Solo Player, Open Battle.net or TCP/IP
    GameLogon               = 0x68, // In Kolbot, sent between "Heartbeat loaded" & "Starting default.dbj"
    EnterGameEnvironment    = 0x69, // This byte should be sent in order to start receiving in-game messages and to interact with the world itself. 

    Ping                    = 0x6D, // This packet should be sent every five to seven seconds to avoid timeout

}


// impl fmt::Display for ClientMessage {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let print = match *self {
// 			ClientMessage::GameLoading              => "GameLoading",
// 		    ClientMessage::WalkToLocation           => "WalkToLocation",
// 		    ClientMessage::WalkToEntity             => "WalkToEntity",           
//             ClientMessage::RunToLocation            => "RunToLocation",
//             ClientMessage::RunToEntity              => "RunToEntity",
//             ClientMessage::LeftSkillOnLocation      => "LeftSkillOnLocation",
//             ClientMessage::LeftSkillOnEntity        => "LeftSkillOnEntity",
//             ClientMessage::LeftSkillOnEntityEx      => "LeftSkillOnEntityEx",
//             ClientMessage::LeftSkillOnLocationEx    => "LeftSkillOnLocationEx",
//             ClientMessage::LeftSkillOnEntityEx2     => "LeftSkillOnEntityEx2",
//             ClientMessage::LeftSkillOnEntityEx3     => "LeftSkillOnEntityEx3",
//             ClientMessage::GameHandshake            => "GameHandshake",
//             ClientMessage::RightSkillOnLocation     => "RightSkillOnLocation",
//             ClientMessage::RightSkillOnEntity       => "RightSkillOnEntity",
//             ClientMessage::RightSkillOnEntityEx     => "RightSkillOnEntityEx",
//             ClientMessage::RightSkillOnLocationEx   => "RightSkillOnLocationEx",
//             ClientMessage::RightSkillOnEntityEx2    => "RightSkillOnEntityEx2",
//             ClientMessage::RightSkillOnEntityEx3    => "RightSkillOnEntityEx3",
//             ClientMessage::InteractWithEntity       => "InteractWithEntity",
//             ClientMessage::OverheadMessage          => "OverheadMessage",
//             ClientMessage::PlayerReassign           => "PlayerReassign",
//             ClientMessage::PickupItem               => "PickupItem",
//             ClientMessage::DropItem                 => "DropItem", 
//             ClientMessage::ItemToBuffer             => "ItemToBuffer",
//             ClientMessage::PickupBufferItem         => "PickupBufferItem",
// 		    ClientMessage::ItemToBody               => "ItemToBody",
// 		    ClientMessage::Swap2HandedItem          => "Swap2HandedItem",
//             ClientMessage::PickupBodyItem           => "PickupBodyItem",
//             ClientMessage::SwitchBodyItem            => "SwitchBodyItem",

//             ClientMessage::UseItem                  => "UseItem",
//             ClientMessage::StackItem                => "StackItem",
//             ClientMessage::RemoveStackItem          => "RemoveStackItem",
//             ClientMessage::ItemToBelt               => "ItemToBelt", 
//             ClientMessage::RemoveBeltItem           => "RemoveBeltItem", 
//             ClientMessage::SwitchBeltItem           => "SwitchBeltItem", 
//             ClientMessage::UseBeltItem              => "UseBeltItem",

//             ClientMessage::InsertSocketItem         => "InsertSocketItem",
//             ClientMessage::ScrollToTome             => "ScrollToTome",
//             ClientMessage::ItemToCube               => "ItemToCube",
//             ClientMessage::UnselectObj              => "UnselectObj",
//             ClientMessage::NpcInit                  => "NpcInit",
//             ClientMessage::NpcCancel                => "NpcCancel",
//             ClientMessage::NpcBuy                   => "NpcBuy",
//             ClientMessage::NpcSell                  => "NpcSell",
//             ClientMessage::NpcTrade                 => "NpcTrade",
//             ClientMessage::CharacterPhrase          => "CharacterPhrase",
            
// 		        ClientMessage::Trade           	     => "Trade",
		    
//             ClientMessage::PortalInfo               => "PortalInfo",
//             ClientMessage::PotionMercenary          => "PotionMercenary", 

//             ClientMessage::GameCreate               => "GameCreate", 
//             ClientMessage::GameLogon                => "GameLogon", 
//             ClientMessage::EnterGameEnvironment     => "EnterGameENvironment", 

//             ClientMessage::Ping                     => "Ping",
            
// 			_							            => "UnknownClientMessage"
//         };
//         write!(f, "{}: {}",self, print)
//     }
// }


/// Packet dispatcher calls the corresponding event handler functions in reaction to a server message
pub fn game_packet_dispatch(_rcv_packet: &D2GSPacket) {
    // TODO how to get rid of this unsafe block without another humongous match{} ?
    // enum has #[repr(u8)] so should'nt be a problem...
    //let dispatch_id: ServerPacketId = unsafe { ::std::mem::transmute(rcv_packet.packet_id()) };
    // match dispatch_id {
    //     ClientMessage::OverheadMessage     	=> chat_event_handler(packet),
    //     ClientMessage::Waypoint
    //                              	=> (),
    //     _ => println!("{}", rcv_packet),
    // }
    //println!("{}", rcv_packet)
}
