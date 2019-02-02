// Raw packet type, ready to be wrapped into a tcp packet!
use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Eq, Clone, Copy)]
pub enum PacketDirection {
	Receive,
	Send
}

impl PartialEq for PacketDirection {
    fn eq(&self, other: &PacketDirection) -> bool {
        self == other
    }
}


// Must impl Ord trait to use std::collections::BinaryHeap as priority queue
#[allow(dead_code)]
#[derive(Eq)]
pub struct RawPacket<'a> {
	// raw client->server packet bytes
	// in case of compressed packets, one raw packet can include several game packets
	raw: &'a[u8],
	direction: PacketDirection,
	priority: u8 // for the priority queue
}


impl<'a> RawPacket<'a> {

	pub fn payload(&self) -> &[u8] {
		self.raw
	}

	pub fn direction(&self) -> PacketDirection {
		self.direction.clone()
	}

	pub fn priority(&self) -> u8 {
		self.priority
	}
}

impl<'a> Ord for RawPacket<'a> {
	fn cmp(&self, other: &RawPacket<'a>) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<'a> PartialOrd for RawPacket<'a> {
    fn partial_cmp(&self, other: &RawPacket<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for RawPacket<'a> {
    fn eq(&self, other: &RawPacket<'a>) -> bool {
        self.raw == other.payload()
    }
}
