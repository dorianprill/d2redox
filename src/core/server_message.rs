/// D2 Game Server Message Identifiers
/// Naming shall follow bnetdocs.org naming unless misleading or wrong
/// Server->Client ClientMessage is determined by the first byte of a D2GSPacket's data
/// The specific arguments argument a packet takes is defined in the respective packet builders
#[repr(u8)]
pub enum ServerMessage {

    GameLoading = 0x00,

    GameFlags {
        difficulty:     u8,
        unknown:        u16,
        hardcore:       u16,
        expansion:      u8,
        ladder:         u8
    } = 0x01,
      
    LoadSuccessful = 0x02,

    LoadAct {
        act:        u8,
        map_id:     u32,
        area_id:    u16,
        unknown:    u32
    } = 0x03,

    LoadComplete = 0x4,

    UnloadComplete = 0x5,

    GameExitSuccessful = 0x06,

    MapReveal {
        tile_x:     u16,
        tile_y:     u16,
        area_id:    u8
    } = 0x07,

    MapHide {
        tile_x:     u16,
        tile_y:     u16,
        area_id:    u8
    } = 0x08,
      
    AssignLevelWarp {
        unit_type:  u8,
        unit_id:    u32,
        unknown:    u8,
        x:          u16,
        y:          u16,
        warp_id:    u8
    } = 0x09,

    RemoveObject {
        unit_type:  u8,
        unit_id:    u32
    } = 0x0A,
      
    GameHandshake {
        unit_type:  u8,
        unit_id:    u32
    } = 0x0B,
      
    NpcHit {
        unit_type:      u8,
        unit_id:        u32,
        animation_id:   u16,
        alive:          u8
    } = 0x0C,
      
    PlayerStop {
        unit_type:  u8,
        unit_id:    u32,
        unknown:    u8,
        x:          u16,
        y:          u16,
        unknown:    u8,
        alive:      u8
    } = 0x0D,

    ObjectState {
        unit_type:  u8,
        unit_id:    u32,
        unknown:    u8,
        unit_state: u32
    } = 0x0E,

    PlayerMove {
        unit_type:  u8,
        unit_id:    u32,
        move_type:  u8,
        target_x:   u16,
        target_y:   u16,
        unknown:    u8,
        current_x:  u16,
        current_y:  u16
    } = 0x0F,

    CharToObject {
        unknown:            u8,
        player_id:          u32,
        movement_type:      u8,
        destination_type:   u8,
        object_id:          u32,
        x:                  u16,
        y:                  u16
    } = 0x10,

    ReportKill {
        unit_type:   u8,
         unit_id:    u32,
         unknown:    u16
    } = 0x11,

    Unknown1 {
        unknown: [u8;25]
    } = 0x12,
       
    Unknown2 {
         unknown: [u8;13]
    } = 0x13,
       
    Unknown3 {
         unknown: [u8;17]
    } = 0x14,

    ReassignPlayer {
        unit_type:   u8,
        unit_id:     u32,
         x:          u16,
         y:          u16,
         value:      u8 
    } = 0x15,
       
    Unknown4 = 0x16,
 
    Unused1 = 0x17,
     
    Unknown5 {
         unknown: [u8;14]
    } = 0x18,

    SmallGoldPickup {
        amount: u8
    } = 0x19,

    AddExpByte {
        amount: u8
    } = 0x1A, 
      
    AddExpWord {
        amount: u16
    } = 0x1B, 
      
    AddExpDWord {
        amount: u32
    } = 0x1C,

    SetByteAttr {
        attribyte:  u8,
        amount:     u8
    } = 0x1D,

    SetWordAttr {
        attribyte:  u8,
        amount:     u16
    } = 0x1E,

    SetDWordAttr {
        attribyte:  u8,
        amount:     u32
    } = 0x1F,

    AttributeUpdate {
        unit_id:    u32,
        attribute:  u8,
        amount:     u32
    } = 0x20,
      
    UpdateItemOSkill {
        unknown1:   u16,
        unit_id:    u32,
        skill:      u16,
        base_level: u8,
        bonus_level:u8,
        unknown2:   u8
    } = 0x21,
      
    UpdateItemSKill {
        unknown1:   u16,
        unit_id:    u32,
        skill:      u16,
        amount:     u8,
        unknown2:   u16
    } = 0x22,
      
    SetSkill {
        unit_type:  u8,
        unit_id:    u32,
        hand:       u8,
        skill:      u16,
        unknown:    u32
    } = 0x23,
      
    Unknown6 {
        unknown: [u8;89] 
    } = 0x24,
      
    Unknown7 {
        unknown: [u8;89]
    } = 0x25,
      
    GameChat {
        chat_kind:  u16,
        unknown:    u16,
        unknown:    u32,
        chat_type:  u8,
        char_name:  &str, // these are C-Strings
        message:    &str
    } = 0x26,

    NpcInfo {
        unit_type:  u8,
        unit_id:    u32,
        unknown:    [u8;34] 
    } = 0x27,
      
    QuestInfo {
        unknown: [u8;102] 
    } = 0x28,
      
    GameQuestInfo {
        unknown:[u8;96] 
    } = 0x29,
      
    NpcTransaction {
        trade_type:         u8,
        result:             u8,
        unknown:            u32,
        merchandise_id:     u32,
        gold_in_inventory:  u32,
    } = 0x2A,
      
    Unused2 = 0x2B, 

    PlaySound {
        unit_type:  u8,
        unit_id:    u32,
        sound:      u16
    } = 0x2C,
      
    Unused3 = 0x2D, 
    Unused4 = 0x2E, 
    Unused5 = 0x2F,
    Unused6 = 0x30,
    Unused7 = 0x31,
    Unused8 = 0x32,
    Unused9 = 0x33,
    Unused10 = 0x34,
    Unused11 = 0x35,
    Unused12 = 0x36,
    Unused13 = 0x37,
    Unused14 = 0x38,
    Unused15 = 0x39,
    Unused16 = 0x3A,
    Unused17 = 0x3B,
    Unused18 = 0x3C,
    Unused19 = 0x3D,

    UpdateItemStats {
        unknown: u8 
    } = 0x3E,
      
    UseStackableItem {
        unknown: [u8;7] 
    } = 0x3F,
      
    Unknown8 {unknown: [u8;12] 
    } = 0x40,
      
    Unused20 = 0x41,

    ClearCursor {
        unit_type: u8,
        player_id: u32
    } = 0x42,
      
    Unused21 = 0x43,

    Unused22 = 0x44,

    Unknown9 {
        unknown: [u8;12] 
    } = 0x45,
      
    Unused23 = 0x46,

    Relator1 {
        param1:     u16,
        unit_id:    u32,
        param2:     u16
    } = 0x47,
      
    Relator2 {
        param1:     u16,
        unit_id:    u32,
        param2:     u16
    } = 0x48,
      
    Unused24 = 0x49,

    Unused25 = 0x4A,

    Unused26 = 0x4B,

    UnitSkillOnTarget {
        unit_type:  u8,
        unit_id:    u32,
        skill:      u16,
        unknown1:   u8,
        unknown2:   u8,
        target_id:  u32,
        unknown3:   u16
    } = 0x4C,
      
    UnitCastSkill {
        unit_type:  u8,
        unit_id:    u32,
        skill:      u16,
        unknown1:   u8,
        x:          u16,
        y:          u16,
        unknown2:   u16
    } = 0x4D,
      
    MercForHire {
        merc_id:    u16,
        unknown:    u32
    } = 0x4E,
      
    StartMercList = 0x4F,

    StartGame = 0x50,

    WorldObject {
        object_type:        u8,
        object_id:          u32,
        object_unique_code: u16,
        x:                  u16,
        y:                  u16,
        state:              u8,
        interact_cond:      u8
    } = 0x51,

    QuestLogInfo {
        unknown: [u8;41]
    } = 0x52,
      
    PlayerSlotRefresh {
        slot:       u32,
        unknown:    u8,
        tick_count: u32
    } = 0x53,
      
    Unknown10 {
           unknown: [u8;9] 
    } = 0x54,
      
    Unknown11 {
        unknown:[u8;2] 
    } = 0x55,
      
    Unused27 = 0x56,
    Unused28 = 0x57,
    
    Unknown12 {
        unknown: [u8;13] 
    } = 0x58,
      
    CreateClientPlayer {
        guid:   u32,
        class:  u8,
        szname: [u8;16],
        x:      u16,
        y:      u16
    } = 0x59,

    EventMessages {
        unknown: [u8;39] 
    } = 0x5A,
      
    PlayerJoined {
        packet_length:  u16,
        player_id:      u32,
        character_class:u8,
        character_name: [u8;16],
        character_level:u16,
        party_id:       u16,
        unknown:        [u8;8] 
    } = 0x5B,

    PlayerLeft {
        player_id: u32
    } = 0x5C,

    QuestItemState {
        unknown: u8,
        state:   u16
    } = 0x5D,
      
    Unknown13 {
        unknown: [u8;37] 
    } = 0x5E,
      
    Unknown14 {
        unknown: [u8;4] 
    } = 0x5F,
      
    TownPortalState {
        state:      u8,
        area_id:    u8,
        unit_id:    u32
    } = 0x60,
      
    Unknown15 {
        unknown: u8
    } = 0x61,
      
    Unknown16 {
        unknown: [u8;6] 
    } = 0x62,
      
    Waypointmenu {
        unit_id:        u32,
        available_wps:  u64
    } = 0x63,
      
    Unused29 = 0x64,

    PlayerKillCount {
        player_id:  u32,
        count:      u16
    } = 0x65,
      
    Unknown17 {
       unknown: [u8;6] 
    } = 0x66,
      
    NpcMove {
        unit_id:    u32,
        unit_type:  u8,
        x:          u16,
        y:          u16,
        unknown1:   u16,
        unknown2:   u8,
        unknown3:   u16,
        unknown4:   u8
    } = 0x67,
      
    NpcMoveToTarget {
        unit_id:            u32,
        unit_type:          u8,
        x:                  u16,
        y:                  u16,
        target_unit_type:   u8,
        target_id:          u32,
        unknown1:           u16,
        unknown2:           u8,
        unknown3:           u16,
        unknown4:           u8
    } = 0x68,

      
    NpcState {
        unit_id:    u32,
        state:      u8,
        x:          u16,
        y:          u16,
        unit_life:  u8,
        unknown:    u8
    } = 0x69,
      
    Unknown18 {
        unknown: [u8;6]
    } = 0x6A,
      
    NpcAction {
        unit_id:    u32,
        action:     u8,
        unknown:    [u8;6],
        x:          u16,
        y:          u16
    } = 0x6B,
      
    NpcAttack {
        unit_id:    u32,
        attack_type:u16,
        target_id:  u32,
        target_type:u8,
        x:          u16,
        y:          u16
    } = 0x6C,
      
    NpcStop {
        unit_id:    u32,
        x:          u16,
        y:          u16,
        unit_life:  u8
    } = 0x6D,
      
    Unknown19 = 0x6E,
    Unknown20 = 0x6F,
    Unknown21 = 0x70,
    Unknown22 = 0x71,
    Unknown23 = 0x72,

    Unknown24 {
        unknown: [u8;31] 
    } = 0x73,
      
    PlayerCorpseAssign {
        assign:     u8,
        owner_id:   u32,
        corpse_id:  u32
    } = 0x74,
      
    PlayerPartyInfo {
        unit_id:        u32,
        party_id:       u16,
        character_level:u16,
        relationship:   u16,
        in_your_party:  u16
    } = 0x75,
      
    PlayerInProximity {
        unit_type:  u8,
        unit_id:    u32
    } = 0x76,
      
    TradeAction {
        request_type: u8
    } = 0x77,

    TradeAccepted {
        character_name: [u8;16],
        unit_id:        u32
    } = 0x78,
      
    GoldInTrade {
        gold_owner: u8, // this is a unit id?
        amount:     u32
    } = 0x79,
      
    LogonResponse {
        unknown: u16
    } = 0x7A,

    AssignSkillHotkey {
        slot:   u8,
        skill:  u8,
        hand:   u8,
        unknown:[u8;4] 
    } = 0x7B,
      
    UseScroll {
        item_type:  u8,
        item_id:    u32
    } = 0x7C,
      
    SetItemState {
        unknown: [u8;17] 
    } = 0x7D,
      
    Unknown25 {
        unknown: [u8;4] 
    } = 0x7E,
      
    AllyPartyInfo {
        unit_type:  u8,
        unit_life:  u16,
        unit_id:    u32,
        unit_area:  u32
    } = 0x7F,
      
    Unused30 = 0x80,

    AssignMerc {
        unknown1:    u8,
        merc_type:  u16,
        player_id:  u32,
        merc_id:    u32,
        unknown2:   u32,
        unknown3:   u32
    } = 0x81,
      
    PortalOwnership {
        player_id:  u32,
        player_name:u32,
        local_id:   u32,
        remote_id:  u32  // different ids on client and server? fun times
    } = 0x82,
      
    Unused31 = 0x83, 
    Unused32 = 0x84,
    Unused33 = 0x85,
    Unused34 = 0x86, 
    Unused35 = 0x87, 
    Unused36 = 0x88,

    UniqueEvents {
        event_id: u8
    } = 0x89,

    NpcWantsToInteract {
        unit_type:  u8,
        unit_id:    u32 
    } = 0x8A,
      
    PlayerRelationship {
        unit_id:    u32,
        state:      u8
    } = 0x8B,
      
    RelationshipUpdate {
        player1_id:     u32,
        player2_id:     u32,
        relationship:   u16 
    } = 0x8C,
      
    AssignPlayerToParty {
        player_id:  u32,
        party_id:   u16
    } = 0x8D,
      
    CorpseAssign {
        assign:     u8,
        player_id:  u32,
        corpse_id:  u32 
    } = 0x8E,
      
    Pong {
        count: [u8;32]
    } = 0x8F,
      
    PartyAutoMapInfo {
        player_id:  u32,
        x:          u32,
        y:          u32
    } = 0x90,
      
    Unknown26 {
        unknown: [u8;25] 
    } = 0x91,
      
    Unknown27 {
        unknown: [u8;5] 
    } = 0x92,
      
    Unknown28 {
        unknown: [u8;7] 
    } = 0x93,
      
    /// TODO verify structure
    /// Skills is an array containing 'amount' items (skills)
    /// each item is comprised of an u16 skill field and a u8 level field
    BaseSkillLevels {
        amount:     u8,
        player_id:  u32,
        skills:     &[SkillDescription]
        //   "skills",
        //   : [
        //     "array",
        //     {
        //       "count": "amount",
        //       : [
        //         "container",
        //         [
        //           {
        //             "skill",
        //             : "lu16"
        //           },
        //           {
        //             "level",
        //             : "lu8"
        //           }
        //         ]
        //       ]
        //     }
        //   ]
    } = 0x94,
      
    /// TODO transcribe sizes into proper masks (in consumer/handler?)
    LifeManaUpdate {
        bitfield: [u8;12]
        //   "life",
        //   "size": 15,
        //   "signed": false
        // },
        // {
        //   "mana",
        //   "size": 15,
        //   "signed": false
        // },
        // {
        //   "stamina",
        //   "size": 15,
        //   "signed": false
        // },
        // {
        //   "x",
        //   "size": 16,
        //   "signed": false
        // },
        // {
        //   "y",
        //   "size": 16,
        //   "signed": false
        // },
        // {
        //   "unknown",
        //   "size": 19,
        //   "signed": false
    } = 0x95,
      
    Walkverify {
        bitfield:   [u8;8]
        //   "stamina",
        //   "size": 15,
        //   "signed": false
        // },
        // {
        //   "x",
        //   "size": 15,
        //   "signed": false
        // },
        // {
        //   "unknown1",
        //   "size": 1,
        //   "signed": false
        // },
        // {
        //   "y",
        //   "size": 15,
        //   "signed": false
        // },
        // {
        //   "unknown2",
        //   "size": 18,
        //   "signed": false
    } = 0x96,
      
    WeaponSwitch = 0x97,

    Unknown29 {
        unknown: [u8;6]
    } = 0x98,
      
    SkillTriggered {
        unknown: [u8;15] 
    } = 0x99,
      
    Unknown30 {
        unknown:[u8;16] 
    } = 0x9A,
      
    MercRelated {
        unknown1: u16,
        unknown2: u32
    } = 0x9B,
      
    /// TODO figure out the count differences
    ItemActionWorld {
        entity_type: u8,
        unknown:     [u8;2] // second u8 is a count with difference 3?     
        // ["array",{
        //     :"lu8",
        //     "countType": ["subtractor",{:"lu8","difference":"3"}]
        //   }]
    } = 0x9C,
      
    ItemActionOwned {
        unknown1:   u8,
        unknown2:   [u8;2] //second field has difference 3
        //   "unknown2",
        //   : ["array",{
        //     :"lu8",
        //     "countType": ["subtractor",{:"lu8","difference":"3"}]
        //     }]
    } = 0x9D,
        
    MercAttributeByte {
        attribute:  u8,
        merc_id:    u32,
        amount:     u8 
    } = 0x9E,
      
    MercAttributeWord {
        attribute:  u8,
        merc_id:    u32,
        amount:     u16 
    } = 0x9F,
      
    MercAttributeDword {
        attribute:  u8,
        merc_id:    u32,
        amount:     u32 
    } = 0xA0,
      
    /// TODO 
    /// This is probably merc_id: u32 and an amount: u8 and an unknown
    MercAddExpByte {
        unknown: [u8;6] 
    } = 0xA1,
      
    /// TODO 
    /// Same as with MercAddExpByte
    MercAddExpWord {
        unknown: [u8;7] 
    } = 0xA2,
      
    Unknown31 {
        unknown: [u8;23] 
    } = 0xA3,
      
    Unknown32 {
        unknown: [u8;2] 
    } = 0xA4,
      
    Unknown33 {
        unknown: [u8;7] 
    } = 0xA5,
      
    Unknown34 = 0xA6,

    /// The delayed state prevents the player from entering another town portal too quickly again
    DelayedState {
        unit_type:  u8,
        unit_id:    u32,
        state:      u8
    } = 0xA7,
      
    SetState {
        unit_type:      u8,
        unit_id:        u32,
        packet_length:  u8, // difference 8
        //   "packetLength",
        //   : ["subtractor", {:"lu8","difference":"8"}]
        state:          u8,
        state_effects:  &[u8] // length of array is determined by packet_length - TODO with substractor or not?
    } = 0xA8,
      
    EndState {
        unit_type:  u8,
        unit_id:    u32,
        state:      u8 
    } = 0xA9,
      
    StateAdd {
        unit_type:  u8,
        guid:       u32,
        // TODO
        // {
        //   "stream",
        //   :  ["array", {
        //     "countType": ["subtractor",{
        //       : "lu8",
        //       "difference": 7
        //     }],
        //     : "lu8"
        //   }]
    } = 0xAA,

    NpcHeal {
        unit_type:  u8,
        unit_id:    u32,
        unit_life:  u8 
    } = 0xAB,

    AssignNpc {
        unit_id:    u32,
        unit_code:  u16,
        x:          u16,
        y:          u16,
        unit_life:  u8,
        state_info: [u8;2] // TODO stateinfo structure
        //   "stateInfo",
        //   : ["array", {
        //     "countType": ["subtractor", {
        //       : "lu8",
        //       "difference": 13
        //     }],
        //     : "lu8"
        //   }]
    } = 0xAC,
      
    Unknown35 {
        unknown: [u8;8] 
    } = 0xAD,
      
    /// TODO 
    /// Figure out what to return in order to fool warden
    /// Else just send exit game packets
    WardenRequest {
        //   "data",
        //   : ["array", {
        //     "countType": "lu16",
        //     : "lu8"
        //   }]
    } = 0xAE,
      

    /// TODO what are the compression modes
    NegotiateCompression {
        mode: u8,
        // TODO 
        // {
        //   "customCompressionData",
        //   : ["switch",{
        //       "compareTo": "compressionMode",
        //       "fields":
        //       {
        //         "0":"void",
        //         "1":"void",
        //         "129":["array", {
        //           "count": 16,
        //           : "lu8"
        //         }]
        //       }
        //     }
    } = 0xAF,

    GameConnectionTerminated = 0xB0,

    Unknown36 {
        unknown: [u8;52] 
    } = 0xB1,
      
    Unknown37 = 0xB2,

    /// TODO what is the argument?
    /// Apparently sent in game before disconnect due to ipban
    Ipban {
        unknown: u32
    } = 0xB3,
      
    Unknown38 = 0xB4, 

    /// TODO is 'message' an array oder an id of a previously sent message?
    OverHead {
        unknown1:   [u8;3],
        unit_type:  u8,
        unit_id:    u32,
        unknown2:   u16,
        unknown3:   u8,
        message:    u8, // is this a message id to be sent before/after?
        unknown4:   u8
    } = 0xB5,

    UnknownFF = 0xFF,
}
      
////////////////////////////////////////////////




// Additional Containers and Bitfields
#[repr(u8)]
struct SkillDescription {
    skill: u16,
    level: u8
}
