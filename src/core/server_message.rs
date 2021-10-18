/// D2 Game Server Message Identifiers
/// I've merged infos from several sources on this:
/// blizzhackers: https://github.com/blizzhackers/Diablo2PacketsData/blob/main/src/data/1.14d/gs2client.json
/// MephisTools: https://github.com/MephisTools/diablo2-protocol/blob/master/data/1.14/d2gs.json
/// ServerMessage (Server->Client) is determined by the first byte of a D2GSPacket's data (enum value here)
#[repr(u8)]
pub enum ServerMessage {

    GameLoading = 0x00,

    /// eArenaFlags contains information about game settings.
    /// eArenaFlags & 0x00000004 = Unknown (Always set)
    /// eArenaFlags & 0x00000800 = Hardcore 
    /// eArenaFlags & 0x00100000 = Expansion
    GameFlags {
        difficulty:     u8,
        eArenaFlags:    u32,
        is_expansion:   u8,
        is_ladder:      u8
    } = 0x01,
      
    LoadSuccessful = 0x02,

    LoadAct {
        act:        u8,
        map_id:     u32,
        area_id:    u16,
        automap:    u32
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
        unit_type:      u8,
        unit_id:        u32,
        warp_class_id:  u8,
        warp_x:         u16,
        warp_y:         u16
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
        animation_id:   u16, // FIXME blizzhackers reports this as two bytes nUnitHitType and nUnitHitClass
        alive:          u8
    } = 0x0C,
      
    PlayerStop {
        unit_type:      u8,
        unit_id:        u32,
        hit_class:      u8,
        x:              u16,
        y:              u16,
        unit_hit_class: u8,
        alive:          u8
    } = 0x0D,

    ObjectState {
        unit_type:      u8,
        unit_id:        u32,
        portal_flags:   u8,
        is_targetable:  u8, // TODO is this flags or bool?
        unit_state:     u32
    } = 0x0E,

    PlayerMove {
        unit_type:      u8,
        unit_id:        u32,
        move_type:      u8,
        target_x:       u16,
        target_y:       u16,
        unit_hit_class: u8,
        current_x:      u16,
        current_y:      u16
    } = 0x0F,

    CharToObject {
        player_type:    u8,
        player_id:      u32,
        movement_type:  u8,
        target_type:    u8,
        target_id:      u32,
        target_x:       u16,
        target_y:       u16
    } = 0x10,

    ReportKill {
        unit_type:  u8,
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
        unit_type:  u8,
        unit_id:    u32,
        x:          u16,
        y:          u16,
        value:      u8   // TODO is this a boolean?
    } = 0x15,
       

    MultipleUnitsCoordsUpdate {
        unused_1:   u8,
        unused_2:   u8,
        count:      u8,
        // then follows an array of units
        // { "sUnitInfo[count]" : [
		//			{ "BYTE" : "nUnitType" },
		//			{ "int" : "nUnitGUID" },
		//			{ "short" : "nUnitX" },
		//			{ "short" : "nUnitY" }
		//		]
		//	}
        // TODO how to represent?
    } = 0x16,
 
    Unused1 = 0x17,
    
    /// BitStream data contains:
    /// HP (15bits)
    /// MP (15bits)
    /// Stamina (15bits)
    /// HPRegen (7bits) 
    /// MPRegen (7bits)
    /// X (16bits)
    /// Y (16bits)
    /// dX (8bits)
    /// dY (8bits)
    /// TODO parse out into own values with crate 'deku'
    HPMPUPDATE {
        packed_bits: [u8;14]
    } = 0x18,

    SmallGoldPickup {
        amount: u8
    } = 0x19,

    AddExpU8 {
        amount: u8
    } = 0x1A, 
      
    AddExpU16 {
        amount: u16
    } = 0x1B, 
      
    AddExpU32 {
        amount: u32
    } = 0x1C,

    SetAttributeU8 {
        attribute:  u8,
        amount:     u8
    } = 0x1D,

    SetAttributeU16 {
        attribute:  u8,
        amount:     u16
    } = 0x1E,

    SetAttributeU32 {
        attribute:  u8,
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
        unknown2:   u8 // FIXME blizzhackers reports this as 'padding'
    } = 0x21,
      
    UpdateItemSkill {
        unknown1:   u16,
        unit_id:    u32,
        skill:      u16,
        amount:     u8,
        unknown2:   u16 // FIXME blizzhackers reports his as two u8's: Padding, bBody
    } = 0x22,
      
    SetSkill {
        unit_type:  u8, // FIXME apparently unused according to blizzhackers
        unit_id:    u32,
        hand:       u8,
        skill_id:   u16,
        item_id:    u32 // FIXME naming according to blizzhackers. maybe signals if skill is from item?
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
        char_name:  &str, // these are C-Strings but..
        message:    &str  // FIXME handle as &String instead?
    } = 0x26,

    NpcInfo {
        unit_type:  u8,
        unit_id:    u32,
        count:      u8,  // TODO is this fixed to value 8?
        unknown:    u8,
        unknown:    [u8;32] 
        // array structure according to blizzhackers:
        //{ "sUnitMessages[8]" : [
		//			{ "BYTE" : "bShow" },
		//			{ "BYTE" : "NotUsed" },
		//			{ "short" : "nMsgID" }
		//		]
		//	}
    } = 0x27,
      
    PlayerQuestInfo {
        update_type:    u8,
        unit_id:        u32,
        action_type:    u8,
        quest_bits:     [u8;96] 
    } = 0x28,
      
    GameQuestInfo {
        unknown:[u8;96] 
    } = 0x29,
      
    NpcTransaction {
        trade_type:     u8,
        result:         u8,
        unused:         u32,
        merchandise_id: u32,
        inventory_gold: u32,
    } = 0x2A,
      
    Unused2 = 0x2B, 

    PlaySound {
        unit_type:  u8,
        unit_id:    u32,
        sound_id:   u16
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
        // TODO according to bh:
        //{ "BYTE" : "nFullPacketSize" },
		//{ "BYTE" : "StatBitStream[nFullPacketSize - 2]" }
    } = 0x3E,
      
    UseStackableItem {
        spell_icon: u8,
        item_id:    u32,
        skill_id:   u16
    } = 0x3F,
      
    SetItemFlags {
        unit_id:    u32,
        item_flag:  u32,
        remove:     u32    
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
        unit_type:  u8,
        gap:        u8,
        unit_id:    u32,
        param2:     u32 // FIXME is this two or four bytes?
    } = 0x47,
      
    Relator2 {
        unit_type:  u8,
        gap:        u8,
        unit_id:    u32,
        param2:     u32 // FIXME is this two or four bytes?
    } = 0x48,
      
    Unused24 = 0x49,

    Unused25 = 0x4A,

    Unused26 = 0x4B,

    UnitSkillOnTarget {
        unit_type:   u8,
        unit_id:     u32,
        skill_id:    u16,
        skill_level: u8,
        target_type: u8,
        target_id:   u32,
        unused:      u16 // all zeros acc. to bh
    } = 0x4C,
      
    UnitSkillOnLocation {
        unit_type:   u8,
        unit_id:     u32,
        skill:       u16,
        unknown1:    u16, // FIXME conflicting info: u8 or u16?
        skill_level: u8,
        x:           u16,
        y:           u16,
        unknown2:    u16 // acc. to bh all zeros
    } = 0x4D,
      
    MercForHire {
        merc_name_id: u16,
        seed:         u32
    } = 0x4E,
      
    StartMercList = 0x4F, // FIXME conflicting info: startmerclist vs clearmerclist

    /// FIXME conflicting info
    /// bh reports:
    /// "D2GS_QUEST_SPECIAL" : {
	///	"PacketId" : "0x50",
	///	"Description" : "",
	///	"Size" : 15,
	///	"Structure" : [ 
	///		{ "BYTE" : "PacketId" }, 
	///		{ "short" : "nMessageType" },
	///		{ "short" : "nArg1" },
	///		{ "short" : "nArg2" },
	///		{ "short" : "nArg3" },
	///		{ "short" : "nArg4" },
	///		{ "short" : "nArg5" },
	///		{ "short" : "nArg6" }
	///	]
	///},
    StartGame = 0x50,

    WorldObject {
        object_type:  u8,
        object_id:    u32,
        object_class: u16,
        x:            u16,
        y:            u16,
        state:        u8,
        interaction:  u8
    } = 0x51,

    PlayerQuestLogInfo {
        unknown: [u8;41]
    } = 0x52,
     
    /// FIXME conflicting info mephi vs bh
    /// PlayerSlotRefresh {
    ///     slot:       u32,
    ///     unknown:    u8,
    ///     tick_count: u32
    /// } = 0x53,
    DarknessUpdate {
        act:    u32,
        angle:  u32,
        on_off: u8,
    } = 0x53,
      
    Unknown10 {
           unknown: [u8;9] 
    } = 0x54,
      
    Unknown11 {
        unknown:[u8;2] 
    } = 0x55,
      
    Unused27 = 0x56,

    NpcEnchants {
        monster_id:      u32,
        monster_type:    u8,
        monster_name_id: u16,
        enchant:         [u8;3],
        unused:          u8,
        champion:        u16, // TODO is this a class? is this a strength multiplier?
    } = 0x57,
    
    /// Conflicting info:
    /// Uknown28 {
    ///     unknown: [u8;13] 
    /// } = 0x58,
    /// BH variant chosen for now
    OpenUI {
        unit_id:    u32,
        ui_type:    u8,
        some_bool:  u8,
    } = 0x58,

  
      
    AssignPlayer {
        unit_id:    u32,
        class:      u8,
        szname:     [u8;16],
        x:          u16,
        y:          u16
    } = 0x59,

    EventMessages {
        message_type:   u8,
        color:          u8,
        arg:            u32,
        arg_type:       u8,
        name1:          [u8;16],
        name2:          [u8;16] 
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

    QuestStateUpdate {
        quest_id:    u8,
        alert_flags: u8,
        status:      u8,
        extra:       u16
    } = 0x5D,
      
    GameQuestAvailability {
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
      
    ActUnlocked {
        act: u8
    } = 0x61,
      
    MakeUnitTargetable {
        unit_type:  u8,
        unit_id:    u32,
        unused:     u8 
    } = 0x62,
      
    WaypointMenu {
        unit_id:        u32,
        unknown:        u16,
        waypoint_bits:  [u8;8],
        unused:         [u8;6]
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
        target_x:   u16,
        target_y:   u16,
        unknown1:   u8,
        unknown2:   u8,
        unknown3:   u8,
        velocity:   u16,
        unknown4:   u8
    } = 0x67,
      

    NpcMoveToEntity {
        unit_id:            u32,
        move_type:          u8,
        target_x:           u16,
        target_y:           u16,
        target_unit_type:   u8,
        target_id:          u32,
        unknown1:           u16,
        unknown2:           u8,
        unused:             u16,
        unknown4:           u8
    } = 0x68,

      
    NpcStateUpdate {
        unit_id:    u32,
        state:      u8,
        x:          u16,
        y:          u16,
        unit_life:  u8,
        hit_class:  u8
    } = 0x69,
      
    /// Some unknown NPC info/interaction/state change
    /// TODO find out what it does
    Unknown18 {
        unit_id:    u32,
        state:      u8,
        unknown1:   u8,
        unknown2:   u32,
        unknown3:   u8
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
        target_x:   u16,
        target_y:   u16
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

    /// Update for missile objects
    /// current_frame might be wrong acc. to bh
    MissileData {
        unused:         u32,
        missile_class:  u16,
        missile_x:      u32,
        missile_y:      u32,
        target_x:       u32,
        target_y:       u32,
        current_frame:  u16,
        owner_type:     u8,
        owner_id:       u32,
        skill_level:    u8,
        pierce_level:   u8 
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
        in_party:       u16
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
        owner_id:   u8,
        amount:     u32
    } = 0x79,
      
    SummonAssign {
        action:         u8,
        skill_id:       u8,
        summon_type:    u16,
        player_id:      u32,
        summon_id:      u32,
    } = 0x7A,

    AssignSkillHotkey {
        slot:       u8,
        skill:      u8,
        hand:       u8,
        item_id:    u32  // for item skills? 
    } = 0x7B,
      
    UseScroll {
        scroll_type:  u8,
        scroll_id:    u32
    } = 0x7C,
      
    SetItemState {
        unit_type:  u8,
        unit_id:    u32,
        item_id:    u32,
        and_value:  u32,
        flags:      u32 
    } = 0x7D,
    
    /// Unknown usage
    CmnCof {
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
        skill_id:    u8,
        summon_type: u16,
        player_id:   u32,
        merc_id:     u32,
        seed2:       u32,
        init_seed:   u32
    } = 0x81,
      
    /// Seems like local_id and remote_id contain 
    /// portals GUID for both ends of each portal.
    /// TODO which one is for town and which one for wilderness?
    PortalOwnership {
        player_id:  u32,
        player_name:u32,
        local_id:   u32,
        remote_id:  u32
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
      
    PlayerPartyUpdate {
        unit_id:     u32,
        party_state: u8
    } = 0x8B,
      
    PlayerRelationUpdate {
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
        pong1:        u32,
        pong2:        u32,
        pong3:        u32,
        count:        u32,
        pong5:        u32,
        pong6_warden: u32,
        pong7_warden: u32,
        pong8_warden: u32
    } = 0x8F,
      
    PlayerMapUpdate {
        player_id:  u32,
        player_x:   u32,
        player_y:   u32
    } = 0x90,
      
    NpcGossip {
        act:    u8,
        unknown: [u16;12] 
    } = 0x91,
      
    ObjectDisplayDisable {
        unit_type:  u8,
        unit_id:    u32
    } = 0x92,
      
    UnknownUnitSkill {
        player_id:  u32,
        unknown1:   u8,
        skill_type: u8,
        skill_page: u8
    } = 0x93,
      
    /// TODO verify structure
    /// Skills is an array containing 'amount' items (skills)
    /// each item is comprised of an u16 skill field and a u8 level field
    PlayerSkillsInfo {
        skills_count:   u8,
        player_id:      u32,
        skills:         &[SkillDescription]
        // actual skill description
        //"sSkills[skills_count]" : [
        //    { "short" : "nSkillId" },
        //    { "BYTE" : "nLevel" }
        //]
    } = 0x94,
      
    /// TODO 
    /// BitStream data contains:
    /// HP (15bits)
    /// MP (15bits) 
    /// Stamina (15bits) 
    /// X (16bits) 
    /// Y (16bits) 
    /// dX (8bits) 
    /// dY (8bits)
    LifeManaUpdate {
        bitfield: [u8;12]
    } = 0x95,
    
    /// TODO
    /// BitStream data contains:
    /// Stamina (15bits) 
    /// X (16bits)
    /// Y (16bits)
    /// dX (8bits) 
    /// dY (8bits)
    WalkUpdate {
        bitfield: [u8;8]
    } = 0x96,
      
    WeaponSwitch = 0x97,

    EvilHut {
        unit_id: u32,
        value:   u16
    } = 0x98,
    

    /// TODO is this for Chance-to-cast skills?
    SkillTriggerOnTarget {
        attacker_type:  u8,
        attacker_id:    u32,
        skill_id:       u16,
        skill_level:    u8,
        target_type:    u8,
        target_id:      u32,
        unused:         u16
    } = 0x99,
    
    
    /// TODO is this for chance-to-cast skills?
    SkillTriggerOnLocation {
        attacker_type:  u8,
        attacker_id:    u32,
        skill_id:       u16,
        unused:         u16,
        skill_level:    u8,
        target_x:       u16,
        target_y:       u16,
        unused2:        u16  

    } = 0x9A,
      
    MercReviveCost {
        merc_name_id: u16,
        revive_cost:  u16,
        unused:       u16
    } = 0x9B,
      
    /// TODO figure out the count differences
    ItemActionWorld {
        action:      u8,
        packet_size: u8,
        category:    u8,
        item_id:     u32,
        bitstream:   &[u8] // { "BYTE" : "BitStream[PacketSize - 8]" }
    } = 0x9C,
      
    ItemActionOwned {
        action:     u8,
        packet_size:u8,
        category:   u8,
        item_id:    u32,
        owner_type: u8,
        owner_id:   u32,
        bitstream:  &[u8] // { "BYTE" : "BitStream[nFullPacketSize - 13]" }
    } = 0x9D,
        
    MercAttributeU8 {
        attribute:  u8,
        merc_id:    u32,
        amount:     u8 
    } = 0x9E,
      
    MercAttributeU16 {
        attribute:  u8,
        merc_id:    u32,
        amount:     u16 
    } = 0x9F,
      
    MercAttributeU32 {
        attribute:  u8,
        merc_id:    u32,
        amount:     u32 
    } = 0xA0,
      
    /// TODO 
    /// This is probably merc_id: u32 and an amount: u8 and an unknown
    MercAddExpU8 {
        stat_id:    u8,
        merc_id:    u32,
        value:      u8 
    } = 0xA1,
      
    /// TODO 
    /// Same as with MercAddExpByte
    MercAddExpU16 {
        stat_id:    u8,
        merc_id:    u32,
        value:      u16 
    } = 0xA2,
    
    /// TODO what does this signal exactly?
    SkillAuraStat {
        aura_stat_id:   u8,
        skill_id:       u16,
        skill_level:    u16,
        unit_type:      u8,
        unit_id:        u32,
        target_type:    u8,
        target_id:      u32,
        target_x:       u32,
        target_y:       u32
    } = 0xA3,
      
    BaalWaves {
        class_id: u16 
    } = 0xA4,
      
    StateSkillMove {
        unit_type:  u8,
        unit_id:    u32,
        skill_id:   u16 
    } = 0xA5,
     
    /// This packet has it's handler in game code 
    /// but it seems like it is never used.
    RunesTxt {
        must_be_zero:   u8,
        packet_size:    u16,
        txt_runes_size: u16,
        bitstream:      &[u8] // { "BYTE" : "BitStream[nFullPacketSize - 6]" }
    } = 0xA6,

    /// The delayed state prevents the player from entering another town portal too quickly again
    DelayState {
        unit_type:  u8,
        unit_id:    u32,
        state:      u8
    } = 0xA7,
      
    SetState {
        unit_type:      u8,
        unit_id:        u32,
        packet_size:    u8,
        state:          u8,
        state_effects:  &[u8] // { "BYTE" : "BitStream[nFullPacketSize - 8]" }
    } = 0xA8,
      
    EndState {
        unit_type:  u8,
        unit_id:    u32,
        state:      u8 
    } = 0xA9,
      
    StateAdd {
        unit_type:   u8,
        unit_id:     u32,
        packet_size: u8,
        bitstream:   &[u8] // { "BYTE" : "BitStream[nFullPacketSize - 7]" }
    } = 0xAA,

    NpcHeal {
        unit_type:  u8,
        unit_id:    u32,
        unit_life:  u8 
    } = 0xAB,

    MonsterAssign {
        unit_id:        u32,
        unit_code:      u16,
        unit_x:         u16,
        unit_y:         u16,
        life_percent:   u8,
        packet_size:    u8,
        bitstream:      &[u8] //{ "BYTE" : "BitStream[nFullPacketSize - 13]" }
    } = 0xAC,
      
    Unknown35 {
        unknown: [u8;8] 
    } = 0xAD,
      
    /// TODO 
    /// Figure out what to return in order to fool warden
    /// Else just send exit game packets
    WardenRequest {
        stream_size: u16,
        bitstream:   &[u8] // { "BYTE" : "Stream[nStreamSize]" }
    } = 0xAE,
      

    /// TODO what are the compression modes
    AdvertiseCompressionMode {
        use_compression: u8,
    } = 0xAF,

    GameConnectionTerminated = 0xB0,

    Unknown36 {
        unknown: [u8;52] 
    } = 0xB1,
      
    GamesInfo{
        unknown1:       [u8;16],
        unknown2:       [u8;16],
        unknown3:       [u8;16],
        clients_count:  u16,
        game_token:     u16

    } = 0xB2,

    /// Apparently sent in game before disconnect to cache character data on client side
    DownloadSave {
        chunk_size: u8,
        first:      u8,
        fillsize:   u32,
        bitstream:  &[u8] //{ "BYTE" : "Stream[nChunkSize]" }
    } = 0xB3,
      
    ConnectionRefused {
        reason: u32
    } = 0xB4, 

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
