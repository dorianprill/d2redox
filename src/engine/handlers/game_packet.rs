use engine::handlers::game_event::*;
use engine::huffman;
/// Server->Client ServerPacket is determined by the first byte of its content i.e. packet[0]
#[allow(dead_code)]
#[repr(u8)]
enum ServerPacket {
    GameLoading         = 0x00,
    GameFlagsPing       = 0x01,
    SetLocale           = 0x02, // what is this really?
    LoadActData         = 0x03,
    NpcUpdate           = 0x0C,
    PlayerMove          = 0x0F,
    PlayerReassign      = 0x15, // this is used for self sent chat packets!?
    Experience1         = 0x1A,
    Experience2         = 0x1B,
    Experience3         = 0x1C,
    ItemSkillBonus1     = 0x21,
    ItemSkillBonus2     = 0x22,
    ChatMessage         = 0x26,
    NPCTransaction      = 0x2A,
    //This message should be used for manipulating the trading window,
    // the Horadric Cube item window, and the Stash window.
    // see https://bnetdocs.org/packet/98/d2gs-trade
    Trade           = 0x4F,
    WorldObject     = 0x51,
    PlayerInit      = 0x59, // Server->Client
    // e.g. 4711 Stones of Jordan Sold to Merchants
    EventMessage    = 0x5A,
    PlayerJoined    = 0x5B,
    PlayerLeft      = 0x5C,
    NPCMoveEntity   = 0x68, // ?
    NPCStateUpdate  = 0x69, // ?
    NPCMoveStop     = 0x6D, // in game i only ever see these received
    MercUpdate      = 0x81,
    PortalUpdate    = 0x82,
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


/// Packet handler calls the corresponding event handler functions in game_event.rs
pub fn game_packet_handler(packet: &[u8]) {
    if packet.len() < 2 || (packet[0] >= 0xF0 && packet.len() < 3) {
        // invalid game packet
        return
    //} else if packet[0] >= 0xF0 { 
    // TODO need to convert compressed packet parsing code
    // Int32 headerSize;
    // Int32 length = Huffman.GetPacketSize(buffer, out headerSize);
    // if (length > buffer.Count)
    //     break;
    //
    // byte[] compressedPacket = buffer.GetRange(headerSize, length).ToArray();
    // buffer.RemoveRange(0, length + headerSize);
    //
    //
    // byte[] decompressedPacket;
    // Huffman.Decompress(compressedPacket, out decompressedPacket);
    // List<byte> packet = new List<byte>(decompressedPacket);
    // while (packet.Count != 0)
    // {
    //     Int32 packetSize;
    //     if (!GetPacketSize(packet, out packetSize))
    //     {
    //         Logger.Write("Failed to determine packet length");
    //         break;
    //     }
    //     List<byte> actualPacket = new List<byte>(packet.GetRange(0, packetSize));
    //     packet.RemoveRange(0, packetSize);
    //
    //     lock (m_connection.Packets)
    //     {
    //         //Logger.Write("Adding a D2GS packet {0:X2}", actualPacket[0]);
    //         m_connection.Packets.Enqueue(1, actualPacket);
    //     }
    //     m_connection.PacketsReady.Set();
    //
    // }

    // Check if received packet is compressed
    let mut result: Vec<u8> = Vec::new();
    let mut which: &[u8] = packet;
    let mut decompress = false;
    if packet[0] >= 0xF0 {
        // and decode the packet
        huffman::decode(packet, result.as_mut()) ;
        which = result.as_mut();
        decompress = true;
    }
    println!(
        "recv d2gs packet len={:04} decompress={:?} {:x?}  {:?}",
        which.len(),
        decompress,
        which,
        String::from_utf8_lossy(which).into_owned()
    );
    // how to get rid of this unsafe block?
    // enum has #[repr(u8)] so should'nt be a problem...
    let header: ServerPacket = unsafe { ::std::mem::transmute(which[0]) };
    match header {
        ServerPacket::SetLocale       => (),
        ServerPacket::PlayerReassign  => (),
        ServerPacket::ChatMessage     => chat_event_handler(&which),
        ServerPacket::NPCTransaction  => (),
        ServerPacket::EventMessage    => (),
        ServerPacket::LifeManaUpdate1
            | ServerPacket::LifeManaUpdate2
                                      => (),
        ServerPacket::ItemAction1
          | ServerPacket::ItemAction2 => (),
        ServerPacket::DelayedStated   => (),
        ServerPacket::WardenScan      => (),
        _ => (),
    }
}
