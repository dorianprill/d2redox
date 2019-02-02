use std::str;

use connection::d2gs::d2gs_packet::D2GSPacket;

pub fn chat_event_handler(packet: &D2GSPacket) {
    // let name: &str    = str::from_utf8(&packet[10..]).unwrap();
    // let message: &str = str::from_utf8(&packet[name.len() + 11..]).unwrap();
    //println!("ChatEvent => {}: {:?}", name, message );
    println!("ChatEvent => {}", str::from_utf8(packet.payload()).unwrap() );
}
