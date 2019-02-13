use std::fmt;
//use std::convert::From;
use engine::handlers::game_event::*;
use connection::d2gs::d2gs_packet::D2GSPacket;
/// Server->Client MessageId is determined by the first byte of its content i.e. packet[0]
#[allow(dead_code)]
#[repr(u8)]
pub enum MessageId {
    GameLoading         = 0x00,
    GameLogonReceipt    = 0x01,
    GameLogonSuccess    = 0x02,
    LoadActData         = 0x03,
    NpcUpdate           = 0x0C,
    PlayerMove          = 0x0F,
    PlayerReassign      = 0x15, // this is used for self sent chat packets!?
    Experience1         = 0x1A,
    Experience2         = 0x1B,
    Experience3         = 0x1C,
    UpdatePlayerSkill       = 0x21,
    UpdatePlayerItemSkill   = 0x22,
    PlayerAssignSkill       = 0x23,

    ChatMessage         = 0x26, // or Game message?
    TransactionComplete = 0x2A,
    // Other unit related actions
    UnitUseSkillOnTarget    = 0x4C,
    UnitUseSkill            = 0x4D,
    // This message should be used for manipulating the trading window,
    // the Horadric Cube item window, and the Stash window.
    // see https://bnetdocs.org/packet/98/d2gs-trade
    Trade               = 0x4F,
    WorldObject         = 0x51,
    PlayerAssign        = 0x59, // Server->Client
    PlayerInformation   = 0x5A,
    PlayerJoined        = 0x5B,
    PlayerLeft          = 0x5C,
    NPCMoveStart        = 0x67,
    NPCMoveToTarget     = 0x68, // ?
    NPCStateUpdate      = 0x69, // ?
    NPCMoveStop         = 0x6D,
    MercUpdate          = 0x81,
    PortalUpdate        = 0x82,
    AssignPlayerToParty = 0x8D,
    // not sure why there are two types for hp/mp update
    LifeManaUpdate1 = 0x18,
    LifeManaUpdate2 = 0x95,
    // again, not sure why there are two types
    ItemAction1     = 0x9C,
    ItemAction2     = 0x9D,
    DelayedStated   = 0xA7,
    NPCAssignment   = 0xAC, // whatever this is
    // warden should'nt find anything since we're not modifying game memory :)
    WardenScan      = 0xAE
}


impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match *self {
			MessageId::GameLoading            => "GameLoading",
		    MessageId::GameLogonReceipt       => "GameLogonReceipt",
		    MessageId::GameLogonSuccess       => "GameLogonSuccess",
		    MessageId::LoadActData            => "LoadActData",
		    MessageId::NpcUpdate              => "NpcUpdate",
		    MessageId::PlayerMove             => "PlayerMove",
		    MessageId::PlayerReassign         => "PlayerReassign",
		    MessageId::Experience1            => "Experience1",
		    MessageId::Experience2            => "Experience2",
		    MessageId::Experience3            => "Experience3",
            MessageId::UpdatePlayerSkill      => "UpdatePlayerSkill",
            MessageId::UpdatePlayerItemSkill  => "UpdatePlayerItemSkill",
            MessageId::PlayerAssignSkill      => "PlayerAssignSkill", 
		    MessageId::ChatMessage            => "ChatMessage",
		    MessageId::TransactionComplete    => "TransactionComplete",
            MessageId::UnitUseSkillOnTarget   => "UnitUseSkillOnTarget",
            MessageId::UnitUseSkill           => "UnitUseSkill",
		    MessageId::Trade           	      => "Trade",
		    MessageId::WorldObject     	      => "WorldObject",
		    MessageId::PlayerAssign           => "PlayerAssign",
		    // e.g. 4711 Stones of Jordan Sold to Merchants
		    MessageId::PlayerInformation  => "PlayerInformation",
		    MessageId::PlayerJoined    	  => "PlayerJoined",
		    MessageId::PlayerLeft      	  => "PlayerLeft",
            MessageId::NPCMoveStart 	  => "NPCMoveStart",
		    MessageId::NPCMoveToTarget 	  => "NPCMoveEntity",
		    MessageId::NPCStateUpdate  	  => "NPCStateUpdate",
		    MessageId::NPCMoveStop     	  => "NPCMoveStop",
		    MessageId::MercUpdate      	  => "MercUpdate",
		    MessageId::PortalUpdate    	  => "PortalUpdate",
		    // not sure why there are two types for hp/mp update
		    MessageId::LifeManaUpdate1 	  => "LifeManaUpdate1",
		    MessageId::LifeManaUpdate2 	  => "LifeManaUpdate1",
		    // again, not sure why there are two types
		    MessageId::ItemAction1     	  => "ItemAction1",
		    MessageId::ItemAction2     	  => "ItemAction2",
		    MessageId::DelayedStated   	  => "DelayedState",
		    MessageId::NPCAssignment   	  => "NPCAssignment",
		    // warden should'nt find anything since we're not modifying game memory :)
		    MessageId::WardenScan         => "WardenScan",
			_							  => "UnknownMessageId"
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
        MessageId::NPCMoveStop      => println!("{}", packet),
        MessageId::LifeManaUpdate1
            | MessageId::LifeManaUpdate2
                                	=> (),
        MessageId::ItemAction1
          | MessageId::ItemAction2 	=> (),
        _ => (),
    }
}
