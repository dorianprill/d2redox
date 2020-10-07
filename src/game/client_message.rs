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

    /// Moves item form the player's cursor buffer to an inventory space (should'nt it be name FromBuffer?)
    ItemToBuffer {
        item_id:    u32, 
        x:          u32, 
        y:          u32, 
        item_buffer:u32
    } = 0x18, 

    /// Pickup an item from the possible buffers below, moving it to the cursor buffer
    PickupBufferItem {
        item_id: u32
    } = 0x19, 

    /// Moves item from player's cursor buffer to body location
    ItemToBody {
        item_id: u32, 
        player_item_slot: u32
    } = 0x1A, 

    /// Moves item from body location to player's cursor buffer
    Swap2HandedItem {
        item_id:          u32, 
        player_item_slot: u32
    } = 0x1B, 

    /// Pickup an item from a Body Location to you're cursor buffer
    PickupBodyItem {
        player_item_slot: u16
    } = 0x1C, 

    /// Swaps item in player's cursor buffer with item in the body location
    SwitchBodyItem {
        item_id: u32, 
        player_item_slot: u32
    } = 0x1D, 

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

    /// DEPRECATED
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
    

    //UnselectObj             = 0x2D, // DEPRECATED

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

    /// This packet's use is currently unconfirmed (bnetdocs)
    /// Possible Trade Types are found in entity::npc::TradeType
    NpcTrade {
        trade_type: u32,
        npc_id:     u32,
        unknown:    u32
    } = 0x38, 

    /// All phrases sent to the server will be 
    /// heard by all players in your vicinity
    /// Possible values for character phrase ids are found in entity::player::PhraseId
    CharacterPhrase {
        phrase_id: u16
    } = 0x3F, 

    /// Requests to go to a waypoint if it was already activated
    Waypoint {
        waypoint_id: u8,
        unknown:     u8,
        unknown2:    u16,
        level_number:u8,
        unknown3:    u16
    } = 0x49, 

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
    
    /// Possible party actions are defined in party::PartyAction
    Party {
        party_action: u16,
        player_id:    u32
    }= 0x5E, 

    //PortalInfo              = 0x60, // TODO

    /// Takes the potion your cursor holds and gives it to the mercenary
    PotionToMercenary {
        uknown: u16
    } = 0x61, 

    /// Replaces 0x67 D2BS_GAMELOGON when creating 
    /// a new Solo Player, Open Battle.net or TCP/IP.
    /// Not used at all on Battle.net.
    /// info taken from https://bnetdocs.org/packet/524/d2gs-gamecreate
    /// Template, Unknown1, Unknown2 & Unknown3 seem to always be 0.
    /// Game name is empty for Solo Player & TCP/IP. 
    /// In Kolbot, sent between "Heartbeat loaded" & "Starting default.dbj"
    GameCreate {
        game_name: [u8;16],
        game_type: u8,
        character_class: u8,
        template_id: u8,
        difficulty:  u8,
        character_name: [u8; 16],
        unknown1:       u16,
        arena_flags:    u32,
        unknown2:       u8,
        unknown3:       u8,
        locale_id:      u8
    } = 0x67, 


    GameLogon               = 0x68, // In Kolbot, sent between "Heartbeat loaded" & "Starting default.dbj"
    EnterGameEnvironment    = 0x69, // This byte should be sent in order to start receiving in-game messages and to interact with the world itself. 

    // This packet should be sent every five to seven seconds to avoid timeout
    Ping {
        tick_count: u32,
        empty:      u32,
        empty2:     u32
    } = 0x6D, 

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
