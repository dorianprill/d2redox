// packets.rs
// this module contains the definitions of the client->server
// and server->client packets and the messages they contain.

use std::fmt;
//use std::convert::From;
//use engine::handlers::game_event::*;
use connection::d2gs::d2gs_packet::D2GSPacket;

pub enum GamePacketId {
    ClientMessage,
    ServerPacketId,
}

impl fmt::Display for GamePacketId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// Packet dispatcher calls the corresponding event handler functions in reaction to a server message
pub fn game_packet_dispatch(_rcv_packet: &D2GSPacket) {
    // TODO how to get rid of this unsafe block without another humongous match{} ?
    // enum has #[repr(u8)] so should'nt be a problem...
    //let dispatch_id: ServerPacketId = unsafe { ::std::mem::transmute(rcv_packet.packet_id()) };
    // match dispatch_id {
    //     ClientMessage::OverheadMessage     	=> chat_event_handler(packet),
    //     ClientMessage::Waypoint
    //                              	=> (),
    //     _ => println!("{}", rcv_packet),
    // }
    //println!("{}", rcv_packet)
}
