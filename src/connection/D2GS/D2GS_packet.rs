/// Packet Structs specificly for D2GS packets
use huffman::*;

/// D2GSPacketCompressed
/// message size can be transmitted in either one u8 or two u8 as
/// depdending on whether the first byte is >= 0xF0
/// Message Layout:
/// if byte[0] < 0xF0:
/// 	(UINT8) Message Size
/// 	(VOID) Message Data
/// if byte[0] >= 0xF0:
/// 	(UINT8) High Message Size
/// 	(UINT8) Low Message Size
/// 	(VOID) Message Data
/// for the latter	message_size := ((High << 8) | Low) & 0xFFF
/// message_size does include itself
/// One compressed package will decode into a &[D2GSPacketPlain] of uncompressed ones
#[allow(dead_code)]
pub struct Compressed {
	message_size: u16,
	message_data: &[u8]
}

impl AsBytes for Compressed{
	pub fn as_bytes(&self) -> &[u8] {
		raw = Vec<u8>::with_capacity(self.message_size);
		raw.push(message_size & 0xFF00);
		raw.push(message_size & 0x00FF);
		raw.push(data) // or? raw + &data or? data.map(|x| x.clone())
		return raw.as_slice()
	}

	pub fn decode(&self) -> &[D2GSPacket] {
		//if packet.len() < 2 || (packet[0] >= 0xF0 && packet.len() < 3) {
	        // invalid game packet
	    	//return
		//}
	    //} else if packet[0] >= 0xF0 {
	    // TODO need to convert compressed packet parsing code
	    // Int32 headerSize;
	    // Int32 length = Huffman.GetPacketSize(buffer, out headerSize);
	    // if (length > buffer.Count)
	    //     break;
	    //
	    // byte[] compressedPacket = buffer.GetRange(headerSize, length).ToArray();
	    // buffer.RemoveRange(0, length + headerSize);
	    //
	    //
	    // byte[] decompressedPacket;
	    // Huffman.Decompress(compressedPacket, out decompressedPacket);
	    // List<byte> packet = new List<byte>(decompressedPacket);
	    // while (packet.Count != 0)
	    // {
	    //     Int32 packetSize;
	    //     if (!GetPacketSize(packet, out packetSize))
	    //     {
	    //         Logger.Write("Failed to determine packet length");
	    //         break;
	    //     }
	    //     List<byte> actualPacket = new List<byte>(packet.GetRange(0, packetSize));
	    //     packet.RemoveRange(0, packetSize);
	    //
	    //     lock (m_connection.Packets)
	    //     {
	    //         //Logger.Write("Adding a D2GS packet {0:X2}", actualPacket[0]);
	    //         m_connection.Packets.Enqueue(1, actualPacket);
	    //     }
	    //     m_connection.PacketsReady.Set();
	    //
	    // }

	    // Check if received packet is compressed
	    let mut result: Vec<u8> = Vec::new();
	    let mut which: &[u8] = packet;
	    let mut decompress = false;
	    if packet.message_id() >= 0xF0 {
	        // and decode the packet
	        huffman::decode(packet, result.as_mut()) ;
	        which = result.as_mut();
	        decompress = true;
	    }
	}
}

#[allow(dead_code)]
pub struct Plain {
	message_id: 	u8,
	message_data:   &[u8]
}

impl AsBytes for Plain {
	pub fn as_bytes(&self) -> &[u8] {
		raw = Vec<u8>::with_capacity(1 + self.message_data.len());
		raw.push(message_id);
		raw.push(data) // or? raw + &data or? data.map(|x| x.clone())
		return raw.as_slice()
	}
}

/// D2GSPacket tagged union
#[allow(dead_code)]
pub enum D2GSPacket {
	Plain,
	Compressed
}

impl D2GSPacket {
//	pub fn from_raw(raw: &[u8]) -> Self {}
	pub fn decode(&self) -> &[D2GSPacket] {
		match self {
			D2GSPacket::Plain => return &self,
			D2GSPacket::Compressed => return self.decode(),
			_ => ()
		}
		else return self.decode() // overwrite in impl?
	}

	pub fn message_id(&self) -> u8 {
		match self {
			D2GSPacket::Plain => return self.message_id,
			_ => ()
		}
	}
}


impl AsBytes for D2GSPacket {
	pub fn as_bytes(&self) -> &[u8] {
		self.as_bytes()
	}
}
