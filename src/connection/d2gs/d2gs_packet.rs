/// decoded D2GS packet data type

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

impl D2GSPacket {
	pub fn new(&self, raw: &[u8]) -> Self {
		return D2GSPacket{data: Vec::from(raw)};
	}
	pub fn packet_id(&self) -> u8 {
		return self.data[0];
	}
	pub fn payload(&self) -> &[u8] {
		return &self.data[1..];
	}
}
