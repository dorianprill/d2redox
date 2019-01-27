use engine::handlers::game_event::*;

/// Server->Client GamePacketType is determined by the first byte of its content i.e. packet[0]
#[repr(u8)]
enum GamePacketType {
    SetLocale       = 0x02,
    PlayerReassign  = 0x15,
    ChatMessage     = 0x26,
    NPCTransaction  = 0x2A,
    // e.g. 4711 Stones of Jordan Sold to Merchants
    EventMessage    = 0x5A,
    // not sure why there are two types for hp/mp update when both is transmitted in the data
    HPUpdate        = 0x18,
    MPUpdate        = 0x95,
    // again, not sure why there are two types
    ItemAction1     = 0x9C,
    ItemAction2     = 0x9D,
    //
    DelayedStated   = 0xA7,
    // should'nt find anything since we're not modifying game memory
    WardenScan      = 0xAE
}


/// Packet handler calls the corresponding event handler functions in game_event.rs
pub fn game_packet_handler(packet: &[u8]) {
    // how to get rid of this unsafe block?
    // enum has #[repr(u8)] so should'nt be a problem...
    let header: GamePacketType = unsafe { ::std::mem::transmute(packet[0]) };
    match header {
        GamePacketType::SetLocale       => (),
        GamePacketType::PlayerReassign  => (),
        GamePacketType::ChatMessage     => chat_event_handler(&packet),
        GamePacketType::NPCTransaction  => (),
        GamePacketType::EventMessage    => (),
        GamePacketType::HPUpdate
          | GamePacketType::MPUpdate    => (),
        GamePacketType::ItemAction1
          | GamePacketType::ItemAction2 => (),
        GamePacketType::DelayedStated   => (),
        GamePacketType::WardenScan      => (),
        _ => (),
    }
}
