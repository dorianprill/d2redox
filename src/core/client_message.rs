/// D2 Game Client Message Identifiers. 
/// Naming shall follow bnetdocs.org naming unless shown to be inappropriate.
/// The specific arguments argument a packet takes is defined in the respective packet builders.
/// The enum discriminants contain the correct packet data types for the corresponding packet type
#[repr(u8)]
pub enum ClientMessage {
    /// Walk to a specified (X,Y) map coordinate
    WalkToLocation {
        x: u16, 
        y: u16
    } = 0x01, 

    /// Makes your character walk to the Entity specified in Entity ID
    WalkToEntity {
        entity_type:    u32, 
        entity_id:      u32
    } = 0x02, 

    /// Run to a specified (X,Y) map coordinate
    RunToLocation {
        x: u16, 
        y: u16
    } = 0x03, 

    /// Makes your character run to the Entity specified in Entity ID
    RunToEntity {
        entity_type:    u32, 
        entity_id:      u32
    } = 0x04, 

    /// Uses Left skill on specified (X,Y) map coordinate
    LeftSkillOnLocation {
        x: u16, 
        y: u16
    } = 0x05, 

    /// Uses your left skill on the Entity specefied in Entity ID
    LeftSkillOnEntity {
        entity_type:    u32, 
        entity_id:      u32
    } = 0x06, 

    /// Uses your left skill on the Entity specefied in Entity ID, 
    /// while holding the hotkey for standing still(shift).
    LeftSkillOnEntityEx {
        entity_type: u32, 
        entity_id: u32
    } = 0x07, 

    /// Uses Left skill on specified (X,Y) map coordinate. 
    /// This packet is sent repeatedly when the mouse button is held 
    /// down after the initial packet has been sent. 
    LeftSkillOnLocationEx {
        x: u16, 
        y: u16
    } = 0x08, 

    /// Uses your left skill on the Entity specified in Entity ID.
    /// This packet is sent repeatedly when the mouse button is held 
    /// down after the initial packet has been sent.
    LeftSkillOnEntityEx2 {
        entity_type: u32, 
        entity_id: u32
    } = 0x09, 

    /// Uses your left skill on the Entity specefied in Entity ID, while holding 
    /// the hotkey for standing still(shift). This packet is sent repeatedly 
    /// when the mouse button is held down after the initial packet has been sent. 
    LeftSkillOnEntityEx3 {
        entity_type: u32, 
        entity_id: u32
    } = 0x0A, 

    /// TODO usage not known, contains no data?
    SkillUnknown = 0x0B,

    ///GameHandshake = 0x0B, // TODO server message?
    /// Uses the currently selected skill at the specified location
    RightSkillOnLocation {
        x: u16, 
        y: u16
    } = 0x0C, 

    /// Uses your right skill on the Entity specefied in Entity ID
    RightSkillOnEntity {
        entity_type: u32, 
        entity_id: u32
    } = 0x0D, 

    /// Uses your right skill on the Entity specefied in Entity ID, while holding the hotkey
    RightSkillOnEntityEx {
        entity_type: u32, 
        entity_id: u32
    } = 0x0E, 

    /// Uses the currently selected skill at the specified location
    RightSkillOnLocationEx {
        x: u16, 
        y: u16
    } = 0x0F, 

    /// Uses your right skill repeatedly on the Entity specified in Entity ID
    RightSkillOnEntityEx2 {
        entity_type: u32, 
        entity_id: u32
    } = 0x10, 

    /// Uses your right skill on the Entity specefied in Entity ID, while holding the hotkey
    RightSkillOnEntityEx3 {
        entity_type: u32, 
        entity_id: u32
    } = 0x11, 

    /// TODO: Not known, contains no data
    SkillUnknown2 = 0x12,

    /// Interacts with the specified Entity. For players and npc's, it will send a request to interact. 
    /// The interaction depends on the type of the unit or object. For others it will trigger the object, for example using a shrine, looting a corpse you have permission to loot, or opening and closing a door.
    InteractWithEntity {
        entity_type: u32, 
        entity_id: u32
    } = 0x13, 

    /// This message is used when you'd like to put a message above a character's head.
    /// Restrictions: 
    ///   Total size of packet cannot be greater than 275 bytes. 
    ///   Message cannot be greater than 255 bytes.
    /// Public colors can be created by adding hex FF 63 and a character 30 to 3C
    OverheadMessage {
        unknown:    u16, 
        message:    String, 
        unused:     u8, 
        unknown2:   u16
    } = 0x14, 

    /// Send a chat message to the server
    /// TODO: what is type?
    ChatMessage {
        msg_type: u8,
        unknown1: u8,
        message: String,
        unknown2: u16
    } = 0x15,

    ///PlayerReassign          = 0x15, // TODO: server message?
    /// Pick up a ground item to cursor buffer/inventory
    PickupItem {
        unit_type:  u32, 
        unit_id:    u32, 
        action_id:  u32
    } = 0x16,

    /// Drops the item in the player's cursor buffer to the ground
    DropItem {
        item_id: u32
    } = 0x17,

    /// Moves item from the player's cursor to an inventory space buffer
    /// possible buffers are found in core::object::item::ItemBufferId
    ItemToBuffer {
        item_id:    u32, 
        x:          u32, 
        y:          u32, 
        item_buffer:u32
    } = 0x18, 

    /// Pickup an item from the possible buffers, moving it to the cursor buffer
    PickupBufferItem {
        item_id: u32
    } = 0x19, 

    /// Moves item from player's cursor buffer to body location
    /// TODO: maybe also works for mercenary
    ItemToBody {
        item_id: u32, 
        player_item_slot: u32
    } = 0x1A, 

    /// Moves item from body location to player's cursor buffer
    /// TODO description is the same as PickupBodyItem, what is the id for?
    Swap2HandedItem {
        item_id:          u32, 
        player_item_slot: u32
    } = 0x1B, 

    /// Pickup an item from a Body Location to your cursor buffer
    PickupBodyItem {
        player_item_slot: u16
    } = 0x1C, 

    /// Swaps item in player's cursor buffer with item in the body location
    /// renamed from SwitchBodyItem (bnetdocs)
    PlaceBodyItem {
        item_id: u32, 
        player_item_slot: u32
    } = 0x1D, 

    /// Swaps item in player's cursor buffer with item in the body location
    /// TODO this message doesn't appear on bnetdocs
    SwitchBodyItem2 {
        item_id: u32,
        player_item_slot: u32
    } = 0x1E,

    /// Tries to swap a cursor item with an item in inventory at position (x,y)
    /// TODO: item_from_id probably needs to be placed on cursor first
    SwitchInventoryItem {
        item_from_id:   u32,
        item_to_id:     u32,
        x:              u32,
        y:              u32
    } = 0x1F,

    /// Uses the specified item (such as a potion, town portal, etc)
    UseItem {
        item_id: u32, 
        x: u32, 
        y: u32
    } = 0x20, 

    /// Stacks one item such as a key onto another item
    StackItem {
        item_id: u32, 
        item_id_target: u32
    } = 0x21,

    /// DEPRECATED according to bnetdocs
    RemoveStackItem {
        item_id: u32
    } = 0x22, 

    /// Moves an item into the player's belt
    ItemToBelt {
        item_id: u32, 
        belt_slot: u32
    } = 0x23, 

    /// Moves the specified item from the belt to the player's cursor buffer
    RemoveBeltItem {
        item_id: u32
    } = 0x24, 

    /// Replaces item in belt with item in player's cursor buffer
    SwitchBeltItem {
        item_id_cursor: u32, 
        item_id: u32
    } = 0x25, 

    /// Uses the specified item in the player's belt.
    UseBeltItem {
        item_id: u32, 
        unknown: u32, 
        unknown2: u32
    }  = 0x26, 

    /// TODO is item1 the scroll and item2 the item to identify?
    IdentifyItem {
        item1_id: u32,
        item2_id: u32
    } = 0x27,

    /// Inserts the specified item into a socketed item
    InsertSocketItem {
        socketable_item: u32, 
        item_id: u32
    } = 0x28, 

    /// Places a scroll into a tome of scrolls
    ScrollToTome {
        item_id_scroll: u32,
        item_id_tome:   u32,
    } = 0x29, 

    /// Moves item from player's cursor buffer into Horadric cube
    ItemToCube {
        item_id: u32,
        cube_id: u32
    } = 0x2A, 
    
    /// DEPRECATED according to bnetdocs
    UnselectObj = 0x2D, 

    /// TODO Unknown, supposedly chat functionality
    ChatUnknown1 {
        unknown1: u16,
    } = 0x2E,

    /// Initiate an NPC sesstion, sent following: C->S 0x13. 
    /// It indicates that you are now interacting with an NPC, and a dialog window is opened. 
    /// This is prior to any choices being made to talk or trade etc.
    NpcInit {
        entity_type: u32,
        entity_id:   u32
    } = 0x2F, 

    /// Stops interacting with an NPC 
    NpcCancel {
        entity_type: u32,
        npc_id: u32 // TODO: also just an entity id?
    } = 0x30, 

    /// TODO 
    QuestMessage {
        unknown1: u32,
        unknown2: u32
    } = 0x31,

    /// Buys an item from a Non Player Character
    NpcBuy {
        npc_id:         u32,
        item_id:        u32,
        buffer_type:    u32,
        cost:           u32
    } = 0x32, 

    /// Sell an item to a Non Player Character
    NpcSell {
        npc_id:         u32,
        item_id:        u32,
        buffer_type:    u32,
        cost:           u32
    } = 0x33, 

    /// TODO what is the argument?
    CainIdentify{
        unknown1: u32
    } = 0x34,
    
    /// TODO what are the ids ?
    Repair {
        id1: u32,
        id2: u32,
        id3: u32,
        id4: u32
    } = 0x35,
    
    /// TODO hat are the ids?
    Hire {
        id1: u32,
        id2: u32
    } = 0x36,
    
    /// TODO dose it choose gamble dialogue if available?
    Gambled {
        id: u32
    } = 0x37,

    /// This packet's use is currently unconfirmed (bnetdocs)
    /// Possible Trade Types are found in entity::npc::TradeType
    NpcTrade {
        trade_type: u32,
        npc_id:     u32,
        unknown:    u32
    } = 0x38, 

    /// TODO what is the type/id ?
    BuyHealth {
        id: u32
    } = 0x39,


    /// TODO make an enum for Strength, Agility, Vitality and Energy values. what are they?
    StatPoint {
        stat_type: u16
    } = 0x3A,

    SkillPoint {
        skill: u16
    } = 0x3B,

    /// TODO what is hand? left or right? values?
    SwitchSkill {
        skill: u8,
        unknown1: u16,
        hand: u8,
        unknown2: [u8;4]
    } = 0x3C,

    /// TODO is parameter an object or an entity?
    CloseDoor {
        unknown: u32
    } = 0x3D,

    /// TODO What is this supposed to do and when to use it?
    UpdateItemStat {
        id: u32
    } = 0x3E,

    /// All phrases sent to the server will be 
    /// heard by all players in your vicinity
    /// Possible values for character phrase ids are found in entity::player::PhraseId
    CharacterPhrase {
        phrase_id: u16
    } = 0x3F, 

    /// TODO no data?
    QuestLog = 0x40,

    /// TODO no data?
    Respawn = 0x41,

    /// TODO what are the ids?
    PutSlot {
        id1: u32,
        id2: u32,
        id3: u32,
        id4: u32
    } = 0x44,

    /// TODO what are the types?
    ChangeTownPortal {
        id1: u32,
        id2: u32,
        id3: u32
    } = 0x45,
      
    /// TODO what is the 'type'? probably attack/getting struck?
    MercenaryInteract {
        merc_id:        u32,
        unit_id:        u32,
        interact_type:  u32
    } = 0x46,

    /// TODO can we really move the merc as we please?
    /// this could be exploitet AF! (always move merc inbetween you and nearest monster, for example)
    MoveMercenary {
        merc_id: u32,
        x:       u32,
        y:       u32
    } = 0x47,

    /// TODO empty?
    Unknown1 = 0x48,

    /// Requests to go to a waypoint if it was already activated
    Waypoint {
        waypoint_id: u8,
        unknown:     u8,
        unknown2:    u16,
        level_number:u8,
        unknown3:    u16
    } = 0x49, 

    /// TODO reassign what? player ids?
    Reassign {
        id1: u32,
        id2: u32
    } = 0x4B,

    /// TODO Probably used for ground items (are they object or entity?)?
    DisappearItem {
        item_id: u32
    } = 0x4C,

    /// TODO
    Unknown2 {
       unknown: u16
    } = 0x4D,

    /// This message should be used for manipulating the trading window,
    /// the Horadric Cube item window, and the Stash window.
    /// see https://bnetdocs.org/packet/98/d2gs-trade
    /// request_id types are found in entity::player::TradeActionId
    Trade {
        request_id:  u32, 
        gold_amount: u16
    } = 0x4F,

    /// Drops a pile of gold to the ground
    DropGold {
        player_id:   u32,
        gold_amount: u32
    } = 0x50,	

    /// TODO what does it do, what are the arguments?
    Assignment {
        unknown1: u32,
        unknown2: u32
    } = 0x51,

    /// TODO what does it do?
    StaOn = 0x53,

    //// TODO what does it do?
    StaOff = 0x54,

    /// TODO questid?
    CloseQuest {
        unknown1: u16
    } = 0x58,

    /// TODO unknown operation
    TownFolk {
        id1: u32,
        id2: u32,
        id3: u32,
        id4: u32
    } = 0x59,

    /// TODO what is the first id and the type?
    Relation {
        id:         u8,
        relation:   u8,
        player_id:  u32
    } = 0x5D,
    
    /// Possible party actions are defined in party::PartyAction
    Party {
        party_action: u16,
        player_id:    u32
    }= 0x5E, 

    /// TODO
    UpdatePosition {
        x: u16,
        y: u16
    } = 0x5F,

    /// TODO verify function
    SwitchEquipment = 0x60,

    /// Takes the potion your cursor holds and gives it to the mercenary
    /// TODO are the arguments correct?
    PotionToMercenary {
        uknown: u16
    } = 0x61, 

    /// Resurrects the mercenary
    /// TODO: need to open npc dialogue first?
    ResurrectMercenary {
        merc_id: u32
    } = 0x62,

    /// TODO data unknown
    /// should have buffer id and belt id?
    InventoryToBelt = 0x63,

    /// Replaces 0x67 D2BS_GAMELOGON when creating 
    /// a new Solo Player, Open Battle.net or TCP/IP.
    /// Not used at all on Battle.net. (TODO is this statement true?)
    /// info taken from https://bnetdocs.org/packet/524/d2gs-gamecreate
    /// Template, Unknown1, Unknown2 & Unknown3 seem to always be 0.
    /// Game name is empty for Solo Player & TCP/IP. 
    /// In Kolbot, sent between "Heartbeat loaded" & "Starting default.dbj"
    GameCreate {
        game_name:      [u8;16],
        game_type:      u8,
        character_class:u8,
        template_id:    u8,
        difficulty:     u8,
        character_name: [u8; 16],
        unknown1:       u16,
        arena_flags:    u32,
        unknown2:       u8,
        unknown3:       u8,
        locale:      u8
    } = 0x67, 

    // In Kolbot, sent between "Heartbeat loaded" & "Starting default.dbj"
    GameLogon {
        mcp_cookie:         u32,
        game_id:            u16,
        character_class:    u8,
        game_version:       u32,
        game_constant:      u64,
        locale:             u8,
        character_name:     [u8;16],

    } = 0x68, 

    /// TODO which of the two definitions is true?
    //EnterGameEnvironment    = 0x69, // This byte should be sent in order to start receiving in-game messages and to interact with the world itself. 
    GameExit = 0x69,


    /// TODO apparently no data
    EnterGameEnvironment = 0x6B,

    // This packet should be sent every five to seven seconds to avoid timeout
    Ping {
        tick_count: u32,
        empty:      u32,
        empty2:     u32
    } = 0x6D, 

}


