use std::str;

pub fn chat_event_handler(packet: &[u8]) {
    let name: &str    = str::from_utf8(&packet[10..]).unwrap();
    let message: &str = str::from_utf8(&packet[name.len() + 11..]).unwrap();
    println!("ChatEvent => {}: {:?}", name, message );
}
