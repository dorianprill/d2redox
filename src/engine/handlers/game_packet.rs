use std::fmt;
//use std::convert::From;
//use engine::handlers::game_event::*;
use connection::d2gs::d2gs_packet::D2GSPacket;

pub enum GamePacketId {
    ClientPacketId,
    ServerPacketId
}

impl fmt::Display for GamePacketId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// D2 Game Server Message Identifiers
/// Naming shall follow bnetdocs.org naming unless shown to be inappropriate
/// Server->Client ClientPacketId is determined by the first byte of a D2GSPacket's data
/// The specific arguments argument a packet takes is defined in the respective packet builders
#[derive(Debug)]
#[allow(dead_code)]
#[repr(u8)]
pub enum ServerPacketId {
    
    CharToObj               = 0x10,
    ReportKill              = 0x11,

    SetGameObjectMode       = 0x14,
    PlayerReassign          = 0x15, // this is used for self sent chat packets!?

    LifeManaUpdate1         = 0x18, //TODO: GS: 15   18 5d 00 16 80 21 40 0b c0 f1 b1 a8 f0 ff 07
    SmallGoldPickup         = 0x19,
    SetByteAttr             = 0x1D,
    SetWordAttr             = 0x1E,
    SetDWordAttr            = 0x1F,
    StateNotification       = 0x20,
    UpdatePlayerSkill       = 0x21,
    UpdatePlayerItemSkill   = 0x22,
    PlayerAssignSkill       = 0x23,

    NPCInteraction          = 0x27, // TODO: parse Merc Info, split in two classes / events: TownFolkInteract and MercInfo.
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
    // This message should be used for manipulating the trading window,
    // the Horadric Cube item window, and the Stash window.
    // see https://bnetdocs.org/packet/98/d2gs-trade
    Trade                   = 0x4F,
    QuestSpecial			= 0x50,	//TODO: length = 15
    WorldObject             = 0x51,
    PlayerQuestLog          = 0x52, //TODO: figure out state values...
    PartyRefresh            = 0x53,

    PlayerAssign            = 0x59, // Server->Client
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
            ServerPacketId::SetWordAttr => "SetWordAttr",
            _ => "UnknownServerPacketId"
        };
        write!(f, "{}: {}", self, print)
    }
}
/// D2 Game Client Message Identifiers
/// Naming shall follow bnetdocs.org naming unless shown to be inappropriate
/// Server->Client ClientPacketId is determined by the first byte of a D2GSPacket's data
/// The specific arguments argument a packet takes is defined in the respective packet builders
#[derive(Debug)]
#[allow(dead_code)]
#[repr(u8)]
pub enum ClientPacketId {
    GameLoading             = 0x00,
    WalkToLocation          = 0x01, // Walk to a specified (X,Y) map coordinate
    WalkToEntity            = 0x02, // Makes your character walk to the Entity specified in Entity ID
    RunToLocation           = 0x03, // Run to a specified (X,Y) map coordinate
    RunToEntity             = 0x04, // Makes your character run to the Entity specified in Entity ID
    LeftSkillOnLocation     = 0x05, // Uses Left skill on specified (X,Y) map coordinate
    LeftSkillOnEntity       = 0x06, // Uses your left skill on the Entity specefied in Entity ID
    LeftSkillOnEntityEx     = 0x07, // Uses your left skill on the Entity specefied in Entity ID, while holding the hotkey for standing still(shift).
    LeftSkillOnLocationEx   = 0x08, // Uses Left skill on specified (X,Y) map coordinate. This packet is sent repeatedly when the mouse button is held down after the initial packet has been sent. 
    LeftSkillOnEntityEx2    = 0x09, // Uses your left skill on the Entity specified in Entity ID
    LeftSkillOnEntityEx3    = 0x0A, // Uses your left skill on the Entity specefied in Entity ID, while holding the hotkey for standing still(shift). This packet is sent repeatedly when the mouse button is held down after the initial packet has been sent. 
    GameHandshake           = 0x0B,
    RightSkillOnLocation    = 0x0C, // Uses the currently selected skill at the specified location
    RightSkillOnEntity      = 0x0D, // Uses your right skill on the Entity specefied in Entity ID
    RightSkillOnEntityEx    = 0x0E, // Uses your right skill on the Entity specefied in Entity ID, while holding the hotkey
    RightSkillOnLocationEx  = 0x0F, // Uses the currently selected skill at the specified location
    RightSkillOnEntityEx2   = 0x10, // Uses your right skill repeatedly on the Entity specified in Entity ID
    RightSkillOnEntityEx3   = 0x11, // Uses your right skill on the Entity specefied in Entity ID, while holding the hotkey
    
    InteractWithEntity      = 0x13, // Interacts with the specified Entity
    OverheadMessage         = 0x14, // This message is used when you'd like to put a message above a character's head
    PlayerReassign          = 0x15, // TODO: this is used for self sent chat packets!?
    PickupItem              = 0x16, // Pick up a ground item to cursor buffer/inventory
    DropItem                = 0x17, // Drops the item in the player's cursor buffer to the ground
    ItemToBuffer            = 0x18, // Moves item form the player's cursor buffer to an inventory space (should'nt it be name FromBuffer?)
    PickupBufferItem        = 0x19, // Pickup an item from the possible buffers below, moving it to the cursor buffer
    ItemToBody              = 0x1A, // Moves item from player's cursor buffer to body location
    Swap2HandedItem         = 0x1B, // Moves item from body location to player's cursor buffer
    PickupBodyItem          = 0x1C, // Pickup an item from a Body Location to you're cursor buffer
    SwitchBodyItem          = 0x1D, // Swaps item in player's cursor buffer with item in the body location
    
    UseItem                 = 0x20, // Uses the specified item (such as a potion, town portal, etc)
    StackItem               = 0x21, // Stacks one item such as a key onto another item
    RemoveStackItem         = 0x22, // DEPRECATED
    ItemToBelt              = 0x23, // Moves an item into the player's belt
    RemoveBeltItem          = 0x24, // Moves the specified item from the belt to the player's cursor buffer
    SwitchBeltItem          = 0x25, // Replaces item in belt with item in player's cursor buffer
    UseBeltItem             = 0x26, // Uses the specified item in the player's belt.
    
    InsertSocketItem        = 0x28, // Inserts the specified item into a socketed item
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


impl fmt::Display for ClientPacketId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match *self {
			ClientPacketId::GameLoading              => "GameLoading",
		    ClientPacketId::WalkToLocation           => "WalkToLocation",
		    ClientPacketId::WalkToEntity             => "WalkToEntity",           
            ClientPacketId::RunToLocation            => "RunToLocation",
            ClientPacketId::RunToEntity              => "RunToEntity",
            ClientPacketId::LeftSkillOnLocation      => "LeftSkillOnLocation",
            ClientPacketId::LeftSkillOnEntity        => "LeftSkillOnEntity",
            ClientPacketId::LeftSkillOnEntityEx      => "LeftSkillOnEntityEx",
            ClientPacketId::LeftSkillOnLocationEx    => "LeftSkillOnLocationEx",
            ClientPacketId::LeftSkillOnEntityEx2     => "LeftSkillOnEntityEx2",
            ClientPacketId::LeftSkillOnEntityEx3     => "LeftSkillOnEntityEx3",
            ClientPacketId::GameHandshake            => "GameHandshake",
            ClientPacketId::RightSkillOnLocation     => "RightSkillOnLocation",
            ClientPacketId::RightSkillOnEntity       => "RightSkillOnEntity",
            ClientPacketId::RightSkillOnEntityEx     => "RightSkillOnEntityEx",
            ClientPacketId::RightSkillOnLocationEx   => "RightSkillOnLocationEx",
            ClientPacketId::RightSkillOnEntityEx2    => "RightSkillOnEntityEx2",
            ClientPacketId::RightSkillOnEntityEx3    => "RightSkillOnEntityEx3",
            ClientPacketId::InteractWithEntity       => "InteractWithEntity",
            ClientPacketId::OverheadMessage          => "OverheadMessage",
            ClientPacketId::PlayerReassign           => "PlayerReassign",
            ClientPacketId::PickupItem               => "PickupItem",
            ClientPacketId::DropItem                 => "DropItem", 
            ClientPacketId::ItemToBuffer             => "ItemToBuffer",
            ClientPacketId::PickupBufferItem         => "PickupBufferItem",
		    ClientPacketId::ItemToBody               => "ItemToBody",
		    ClientPacketId::Swap2HandedItem          => "Swap2HandedItem",
            ClientPacketId::PickupBodyItem           => "PickupBodyItem",
            ClientPacketId::SwitchBodyItem            => "SwitchBodyItem",

            ClientPacketId::UseItem                  => "UseItem",
            ClientPacketId::StackItem                => "StackItem",
            ClientPacketId::RemoveStackItem          => "RemoveStackItem",
            ClientPacketId::ItemToBelt               => "ItemToBelt", 
            ClientPacketId::RemoveBeltItem           => "RemoveBeltItem", 
            ClientPacketId::SwitchBeltItem           => "SwitchBeltItem", 
            ClientPacketId::UseBeltItem              => "UseBeltItem",

            ClientPacketId::InsertSocketItem         => "InsertSocketItem",
            ClientPacketId::ScrollToTome             => "ScrollToTome",
            ClientPacketId::ItemToCube               => "ItemToCube",
            ClientPacketId::UnselectObj              => "UnselectObj",
            ClientPacketId::NpcInit                  => "NpcInit",
            ClientPacketId::NpcCancel                => "NpcCancel",
            ClientPacketId::NpcBuy                   => "NpcBuy",
            ClientPacketId::NpcSell                  => "NpcSell",
            ClientPacketId::NpcTrade                 => "NpcTrade",
            ClientPacketId::CharacterPhrase          => "CharacterPhrase",
            
		    ClientPacketId::Trade           	     => "Trade",
		    
            ClientPacketId::PortalInfo               => "PortalInfo",
            ClientPacketId::PotionMercenary          => "PotionMercenary", 

            ClientPacketId::GameCreate               => "GameCreate", 
            ClientPacketId::GameLogon                => "GameLogon", 
            ClientPacketId::EnterGameEnvironment     => "EnterGameENvironment", 

            ClientPacketId::Ping                     => "Ping",
            
			_							        => "UnknownClientPacketId"
        };
        write!(f, "{}: {}",self, print)
    }
}


/// Packet handler calls the corresponding event handler functions in game_event.rs
pub fn game_packet_dispatch(packet: &D2GSPacket) {
    // TODO how to get rid of this unsafe block without another humongous match{} ?
    // enum has #[repr(u8)] so should'nt be a problem...
    //let dispatch_id: ClientPacketId = unsafe { ::std::mem::transmute(packet.packet_id()) };
    //match dispatch_id {
        //ClientPacketId::OverheadMessage     	=> chat_event_handler(packet),
    //    ClientPacketId::Waypoint
    //                             	=> (),
    //    _ => println!("{}", packet),
    //}
}
