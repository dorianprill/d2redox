use std::str;

use connection::d2gs::d2gs_packet::D2GSPacket;


pub fn chat_event_handler(packet: &D2GSPacket) {
    // let name: &str    = str::from_utf8(&packet[10..]).unwrap();
    // let message: &str = str::from_utf8(&packet[name.len() + 11..]).unwrap();
    //println!("ChatEvent => {}: {:?}", name, message );
    println!("{}", packet);
}

pub fn item_action_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}

pub fn world_obejct_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}

pub fn player_init_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}

pub fn player_move_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}

pub fn player_reassign_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}

pub fn player_joined_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}

pub fn player_left_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}

pub fn life_mana_update_handler(packet: &D2GSPacket) {
	println!("{}", packet);
}
