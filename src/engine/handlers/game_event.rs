use std::str;

use d2re::connection::d2gs::D2GSPacket;

pub fn chat_event_handler(packet: &[D2GSPacket]) {
    let name: &str    = str::from_utf8(&packet[10..]).unwrap();
    let message: &str = str::from_utf8(&packet[name.len() + 11..]).unwrap();
    println!("ChatEvent => {}: {:?}", name, message );
}
