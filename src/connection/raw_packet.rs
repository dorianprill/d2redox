// Raw packet type, ready to be wrapped into a tcp packet!
use std::cmp::Ordering;

#[allow(dead_code)]
pub enum PacketDirection {
	Receive,
	Send
}

// We need to derive Hash and Eq for use with PriorityQueue from crate
// maybe impl Ord trait and use std::collections::BinaryHeap instead of crate?
//#[derive(Hash, Eq)]
#[allow(dead_code)]
pub struct RawPacket {
	// raw client->server packet bytes
	// in case of compressed packets, one raw packet can include several game packets
	raw: &[u8],
	direction: PacketDirection,
	priority: u8 // for the priority queue
}


impl RawPacket {
	pub fn payload(&self) -> &[u8] {
		return self.raw
	}

	pub fn priority(&self) -> u8 {
		return self.priority
	}
}

impl Ord for RawPacket {
	fn cmp(&self, other: &RawPacket) -> Ordering {
        self.priority.cmp(&other.height)
    }
}
