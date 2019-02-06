/// decoded D2GS packet data type
use std::fmt;
use engine::handlers::game_packet::MessageId;

pub trait AsBytes {
	fn as_bytes(&self) -> &[u8];
}

#[allow(dead_code)]
pub struct D2GSPacket {
	pub data: Vec<u8>,
}

impl AsBytes for D2GSPacket {
	fn as_bytes(&self) -> &[u8] {
		return self.data.as_slice();
	}
}

impl fmt::Display for D2GSPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Values:\n")?;
		let id: MessageId = unsafe { ::std::mem::transmute(self.packet_id()) };
		write!(f, "{} (0x{}) Payload: , ", id, self.packet_id())?;
        for v in &self.data {
            write!(f, "{}, ", v)?;
        }
        Ok(())
    }
}


impl D2GSPacket {

	pub fn packet_id(&self) -> u8 {
		return self.data[0];
	}
	pub fn payload(&self) -> &[u8] {
		return &self.data[1..];
	}
}
