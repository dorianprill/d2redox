use std::fmt;
//use std::convert::From;
use engine::handlers::game_event::*;
use connection::d2gs::d2gs_packet::D2GSPacket;
/// Server->Client MessageId is determined by the first byte of its content i.e. packet[0]
#[allow(dead_code)]
#[repr(u8)]
pub enum MessageId {
    GameLoading             = 0x00,
    GameLogonReceipt        = 0x01,
    GameLogonSuccess        = 0x02,
    ActDataLoad             = 0x03,
    ActDataComplete         = 0x04,
    UnloadDone              = 0x05,
    GameLogoutSuccess       = 0x06,
    MapAdd                  = 0x07,
    MapRemove               = 0x08,
    AssignWarp              = 0x09,
    RemoveGroundUnit        = 0x0A,
    GameHandshake           = 0x0B,
    NpcUpdate               = 0x0C,
    PlayerStop              = 0x0D,
    GameObjectModeUpdate    = 0x0E,
    PlayerMove              = 0x0F,
    PlayerMoveToUnit        = 0x10,
    ReportKill              = 0x11,

    SetGameObjectMode       = 0x14,
    PlayerReassign          = 0x15, // this is used for self sent chat packets!?

    LifeManaUpdate1         = 0x18, //TODO: GS: 15   18 5d 00 16 80 21 40 0b c0 f1 b1 a8 f0 ff 07
    SmallGoldAdd            = 0x19,
    Experience1             = 0x1A,
    Experience2             = 0x1B,
    Experience3             = 0x1C,
    AttributeByte           = 0x1D,
    AttributeWord           = 0x1E,
    AttributeDWord          = 0x1F,
    StateNotification       = 0x20,
    UpdatePlayerSkill       = 0x21,
    UpdatePlayerItemSkill   = 0x22,
    PlayerAssignSkill       = 0x23,

    ChatMessage             = 0x26, // or is it Game message/announcement?
    NPCInteraction          = 0x27, //TODO: parse Merc Info, split in two classes / events: TownFolkInteract and MercInfo.
    PlayerQuestInfo         = 0x28,
    UpdateGameQuestLog      = 0x29,
    TransactionComplete     = 0x2A,

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
    //UNKOWN			      = 0x50,	//TODO: length = 15
    WorldObject             = 0x51,
    PlayerQuestLog          = 0x52, //TODO: figure out state values...
    PartyRefresh            = 0x53,

    PlayerAssign            = 0x59, // Server->Client
    PlayerInformation       = 0x5A,
    PlayerJoined            = 0x5B,
    PlayerLeft              = 0x5C,
    //UNKOWN				  = 0x5D,	//TODO: length = 6
    //UnknownGame			  = 0x5E,
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

    GameOver                = 0xB0,
    Invalid                 = 0xB1,

    Wrapper                 = 0xF0,

}


impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match *self {
			MessageId::GameLoading              => "GameLoading",
		    MessageId::GameLogonReceipt         => "GameLogonReceipt",
		    MessageId::GameLogonSuccess         => "GameLogonSuccess",           
            MessageId::ActDataLoad              => "ActDataLoad",
            MessageId::ActDataComplete          => "ActDataComplete",
            MessageId::UnloadDone               => "UnloadDone",
            MessageId::GameLogoutSuccess        => "GameLogoutSuccess",
            MessageId::MapAdd                   => "MapAdd",
            MessageId::MapRemove                => "MapRemove",
            MessageId::AssignWarp               => "AssignWarp",
            MessageId::RemoveGroundUnit         => "RemoveGroundUnit",
            MessageId::GameHandshake            => "GameHandshake",
            MessageId::NpcUpdate                => "NpcUpdate",
            MessageId::PlayerStop               => "PlayerStop",
            MessageId::GameObjectModeUpdate     => "GameObjectModeUpdate",
            MessageId::PlayerMove               => "PlayerMove",
            MessageId::PlayerMoveToUnit         => "PlayerMoveToUnit",
            MessageId::ReportKill               => "ReportKill",
            MessageId::SetGameObjectMode        => "SetGameObjectMode",
            MessageId::PlayerReassign           => "PlayerReassign",
            MessageId::LifeManaUpdate1 	        => "LifeManaUpdate1",
            MessageId::SmallGoldAdd             => "SmallGoldAdd",
		    MessageId::Experience1              => "Experience1",
		    MessageId::Experience2              => "Experience2",
            MessageId::Experience3              => "Experience3",
            MessageId::AttributeByte            => "AttributeByte",
            MessageId::AttributeWord            => "AttributeWord",
            MessageId::AttributeDWord           => "AttributeDWord",
            MessageId::StateNotification        => "StateNotification",
            MessageId::UpdatePlayerSkill        => "UpdatePlayerSkill",
            MessageId::UpdatePlayerItemSkill    => "UpdatePlayerItemSkill",
            MessageId::PlayerAssignSkill        => "PlayerAssignSkill", 
            MessageId::ChatMessage              => "ChatMessage",
            MessageId::NPCInteraction           => "NPCInteraction",
            MessageId::PlayerQuestInfo          => "PlayerQuestInfo",
            MessageId::UpdateGameQuestLog       => "UpdateGameQuestLog",
            MessageId::TransactionComplete      => "TransactionComplete",
            MessageId::PlaySound                => "PlaySound",
            MessageId::UpdateContainerItem      => "UpdateContainerItem",
            MessageId::UseStackableItem         => "UseStackableItem",
            MessageId::PlayerClearCursor        => "PlayerClearCursor",
            MessageId::Relator1                 => "Relator1",
            MessageId::Relator2                 => "Relator2",
            MessageId::UnitUseSkillOnTarget     => "UnitUseSkillOnTarget",
            MessageId::UnitUseSkill             => "UnitUseSkill",
            MessageId::MercForHire              => "MercForHire",
		    MessageId::Trade           	        => "Trade",
            MessageId::WorldObject     	        => "WorldObject",
            MessageId::PlayerQuestLog           => "PlayerQuestLog", //TODO: figure out state values...
            MessageId::PartyRefresh             => "PartyRefresh",
		    MessageId::PlayerAssign             => "PlayerAssign",
		    // e.g. 4711 Stones of Jordan Sold to Merchants
		    MessageId::PlayerInformation        => "PlayerInformation",
		    MessageId::PlayerJoined    	        => "PlayerJoined",
            MessageId::PlayerLeft      	        => "PlayerLeft",
            MessageId::PortalInfo               => "PortalInfo",
            MessageId::OpenWaypoint             => "OpenWaypoint",
            MessageId::PlayerKillCount          => "PlayerKillCount",
            MessageId::NpcMoveStart 	        => "NPCMoveStart",
		    MessageId::NpcMoveToTarget 	        => "NPCMoveEntity",
            MessageId::NpcStateUpdate  	        => "NPCStateUpdate",
            MessageId::NpcAction                => "NPCAction",
            MessageId::NpcAttack                => "NPCAttack",
            MessageId::NpcMoveStop     	        => "NPCMoveStop",
            MessageId::AboutPlayer              => "AboutPlayer",
            MessageId::OverHeadClear            => "OverHeadClear",
            MessageId::UpdateItemUI             => "UpdateItemUI",
            MessageId::TradeAccept              => "TradeAccept",
            MessageId::TradeGold                => "TradeGold",
            MessageId::SummonAction             => "SummonAction",
            MessageId::AssignSkillHotkey        => "AssignSkillHotkey",
            MessageId::UseSpecialItem           => "UseSpecialItem",	 //TODO: Only type 4 : Identify / portal tome / scroll is known
            MessageId::PartyMemberUpdate        => "PartyMemberUpdate",
		    MessageId::MercUpdate      	        => "MercUpdate",
            MessageId::PortalUpdate    	        => "PortalUpdate",
            MessageId::NpcWantsInteract         => "NpcWantsInteract",
            MessageId::PlayerPartyRelation      => "PlayerPartyRelation",
            MessageId::PlayerRelation           => "PlayerRelation",
            MessageId::AssignPlayerToParty      => "AssignPlayerToParty",
            MessageId::CorpseAssign             => "CorpseAssign",
            MessageId::Pong                     => "Pong",
            MessageId::PartyMemberPulse         => "PartMemberPulse",
		    MessageId::SkillsLog                => "SkillsLog",
            MessageId::PlayerLifeManaChange     => "PlayerLifeManaChange", // not sure why there are two types for hp/mp update (0x18 and 0x95)
            MessageId::WalkVerify               => "WalkVerify",
		    MessageId::ItemWorldAction          => "ItemWorldAction",
            MessageId::ItemOwnedAction          => "ItemOwnedAction",
            MessageId::MercAttributeByte        => "MercAttributeByte",
            MessageId::MercAttributeWord        => "MercAttributeWord",
            MessageId::MercAttributeDWord       => "MercAttributeDWord",
            MessageId::DelayedState   	        => "DelayedState",
            MessageId::SetState                 => "SetState",
            MessageId::EndState                 => "EndState",
            MessageId::AddUnit                  => "AddUnit",
            MessageId::NpcHeal                  => "NpcHeal",
		    MessageId::NpcAssignment   	        => "NPCAssignment",
		    // warden should'nt find anything since we're not modifying game memory :)
		    MessageId::WardenScan               => "WardenScan",
			_							        => "UnknownMessageId"
        };
        write!(f, "{}", print)
    }
}


/// Packet handler calls the corresponding event handler functions in game_event.rs
pub fn game_packet_dispatch(packet: &D2GSPacket) {

    // println!(
    //     "recv d2gs packet len={:04} decompress={:?} {:x?}  {:?}",
    //     which.len(),
    //     decompress,
    //     which,
    //     String::from_utf8_lossy(which).into_owned()
    // );
    // TODO how to get rid of this unsafe block without another humongous match{} ?
    // enum has #[repr(u8)] so should'nt be a problem...
    let dispatch_id: MessageId = unsafe { ::std::mem::transmute(packet.packet_id()) };
    match dispatch_id {
        MessageId::ChatMessage     	=> chat_event_handler(packet),
        // MessageId::LifeManaUpdate1
        //     | MessageId::PlayerLifeManaChange
        //                         	=> (),
        _ => println!("{}", packet),
    }
}
