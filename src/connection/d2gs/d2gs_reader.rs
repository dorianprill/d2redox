use connection::d2gs::d2gs_packet::D2GSPacket;
use connection::huffman;
use engine::handlers::game_packet::game_packet_dispatch;

use std::collections::VecDeque;

const PACKET_SIZES: [i32; 177] = [
			// 1, 8, 1, 12, 1, 1, 1, 6, 6, 11, 6, 6, 9, 13, 12, 16,
			// 16, 8, 26, 14, 18, 11, 0, 0, 15, 2, 2, 3, 5, 3, 4, 6,
			// 10, 12, 12, 13, 90, 90, 0, 40, 103,97, 15, 0, 8, 0, 0, 0,
			// 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 8,
			// 13, 0, 6, 0, 0, 13, 0, 11, 11, 0, 0, 0, 16, 17, 7, 1,
			// 15, 14, 42, 10, 3, 0, 0, 14, 7, 26, 40, 0, 5, 6, 38, 5,
			// 7, 2, 7, 21, 0, 7, 7, 16, 21, 12, 12, 16, 16, 10, 1, 1,
			// 1, 1, 1, 32, 10, 13, 6, 2, 21, 6, 13, 8, 6, 18, 5, 10,
			// 4, 20, 29, 0, 0, 0, 0, 0, 0, 2, 6, 6, 11, 7, 10, 33,
			// 13, 26, 6, 8, 0, 13, 9, 1, 7, 16, 17, 7, 0, 0, 7, 8,
			// 10, 7, 8, 24, 3, 8, 0, 7, 0, 7, 0, 7, 0, 0, 0, 0,
			// 1 ];

			1, 9, 1, 12, 1, 1, 1, 6, 6, 11, 6, 6, 9, 13, 12, 16,
/* 1 */		16, 8, 26, 14, 18, 11, -1, -1, 15, 2, 2, 3, 5, 3, 4, 6,
/* 2 */		10, 12, 12, 13, 90, 90, -1, 40, 103, 97, 15, 0, 8, 0, 0, 0,
/* 3 */		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 8,
/* 4 */		13, 0, 6, 0, 0, 13, 0, 11, 11, 0, 0, 0, 16, 17, 7, 1,
/* 5 */		15, 14, 42, 10, 3, 0, 0, 14, 7, 26, 40, -1, 5, 6, 38, 5,
/* 6 */		7, 2, 7, 21, 0, 7, 7, 16, 21, 12, 12, 16, 16, 10, 1, 1,
/* 7 */		1, 1, 1, 32, 10, 13, 6, 2, 21, 6, 13, 8, 6, 18, 5, 10,
/* 8 */		4, 20, 29, 0, 0, 0, 0, 0, 0, 2, 6, 6, 11, 7, 10, 33,
/* 9 */		13, 26, 6, 8, -1, 13, 9, 1, 7, 16, 17, 7, -1, -1, 7, 8,
/* A */		10, 7, 8, 24, 3, 8, -1, 7, -1, 7, -1, 7, -1, 0, -1, 0,
/* B */		1 ];


pub struct D2GSReader {
    // used as a single ended queue here
    packets: VecDeque<D2GSPacket>,
}

impl D2GSReader {

    pub fn new() -> Self {
        return D2GSReader{packets: VecDeque::new()};
    }

	pub fn next(&mut self) -> Option<D2GSPacket> {
        return self.packets.pop_front();
    }

    /// read() handles the D2GS packet compression
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
    /// One compressed package will decode into several uncompressed ones
    pub fn read(&mut self, raw: &[u8]) {
		// else is compressed packet
		let mut decompressed_chunk = Vec::with_capacity(raw.len());
		let mut start: 		usize = 0;
		let mut nheader: 	usize = 0;
		let mut ndata: 		usize = 0;
		let mut end:		usize = 0;

		while (start + nheader + ndata) < raw.len() {

			// is invalid packet
			if raw[start..].len() < 2 || (raw[start] >= 0xF0 && raw[start..].len() < 3) {
				println!("D2GSReader::read(): input too short");
				dbg!(ndata);
				dbg!(nheader);
				dbg!(raw[start..].len());
				dbg!(raw.len());
				dbg!(raw[0]);
				return
			}

			// is plain packet
			if raw[start] < 0xF0 {
				self.packets.push_back(D2GSPacket{data: raw.to_vec()});
				return
			}

			// else is compressed packet
			ndata = huffman::get_chunk_params(&raw[start..], &mut nheader);
			if ndata > raw[start..].len() {
				// something went wrong
				println!("D2GSReader::read(): invalid chunk params");
				dbg!(ndata);
				dbg!(nheader);
				dbg!(raw[start..].len());
				dbg!(raw.len());
				dbg!(raw[0]);
	            return
			}
			// index needs -1 since start is already header[0]?
			end = start+nheader+ndata;
			huffman::decompress(&raw[start+nheader..end], &mut decompressed_chunk); // ..end not included!
			start = end+1; // proceed with next chunk
			while decompressed_chunk.len() > 0 {
				let mut actual_size: i32 = 0;
				if !get_packet_size(&decompressed_chunk, &mut actual_size) {
					println!("D2GSReader::read(): failed to determine packet length");
                    return
                }
            	self.packets.push_back(D2GSPacket{data: decompressed_chunk.drain(0..actual_size as usize).collect()});
			}

		}

		// After internal packet queue is filled, pop & handle all
		self.handle_all();
    }


	pub fn handle_all(&mut self) {
		while let Some(p) = self.packets.pop_front() {
			game_packet_dispatch(&p);
		}
	}

} // impl D2GSReader


// translated from OmegaBot
pub fn get_chat_packet_size(input: &[u8], out: &mut i32) -> bool {
	let mut output: i32 = 0;
	if input.len() < 12 {
		return false;
	}

	const initial_offset: i32 = 10;
	let mut name_offset: i32 = input
						.iter()
						.position(|&x| (x as i32) == initial_offset)
						.unwrap() as i32;

	if name_offset == -1 {
		return false;
	}
	name_offset -= initial_offset;

	let mut message_offset: i32 = input
						.iter()
						.position(|&x| (x as i32) == (initial_offset + name_offset + 1))
						.unwrap() as i32;

	if message_offset == -1 {
		return false;
	}

	message_offset = message_offset - initial_offset - name_offset -1;
	output = initial_offset + name_offset + 1 + message_offset + 1;

	return true;
}

// This was taken from Redvex according to qqbot source
pub fn get_packet_size(input: &[u8], output: &mut i32) -> bool {
	let identifier: u8 = input[0];
	let size = input.len() as i32;

	match identifier {
		0x26 =>
			if get_chat_packet_size(input, output ) {
				return true;
			}
		0x5B =>
			if size >= 3 {
				*output = ((input[1] as i32) << 8) & input[2] as i32;
				return true;
			}
		0x94 =>
			if size >= 2 {
				*output = input[1] as i32 * 3 + 6;
				return true;
			}
		0xA8
		| 0xAA =>
			if size >= 7 {
				*output = input[6] as i32;
				return true;
			}
		0xAC =>
			if size >= 13 {
				*output = input[12] as i32;
				return true;
			}
		0xAE =>
			if size >= 3 {
				*output = 3 + ((input[1] as i32) << 8) & input[2] as i32;
				return true;
			}
		0x9C =>
			if size >= 3 {
				*output = input[2] as i32;
				return true;
			}
		0x9D =>
			if size >= 3 {
				*output = input[2] as i32;
				return true;
			}
		_ =>
			if identifier < PACKET_SIZES.len() as u8 {
				*output = PACKET_SIZES[identifier as usize] as i32;
				return *output != 0;
			}
	}
	*output = 0;
	return false;
}
