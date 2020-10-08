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
