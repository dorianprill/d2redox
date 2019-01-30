/// Packet Struct specificly for D2GS packets

#[allow(dead_code)]
pub struct D2GSPacket {
	header: u8,
	len:    u16,
	data:   &[u8]
}

impl AsRaw for D2GSPacket {
	pub fn as_raw(&self) -> &[u8] {
		raw = Vec<u8>::with_capacity(1 + self.len + self.data.len());
		raw.push(header);
		// x86[_64] is always little endian, so this is no problem
		raw.push(len & 0xFF00);
		raw.push(len & 0x00FF);
		raw.push(data) // or? raw + &data or? data.map(|x| x.clone())
		return raw.as_slice()
	}
}
