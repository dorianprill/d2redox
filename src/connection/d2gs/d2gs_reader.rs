use connection::d2gs::d2gs_packet::D2GSPacket;
use connection::huffman;
use engine::handlers::game_packet::game_packet_dispatch;

use std::collections::VecDeque;

const KNOWN_LENGTHS: [u16; 177] = [
			1, 8, 1, 12, 1, 1, 1, 6, 6, 11, 6, 6, 9, 13, 12, 16,
			16, 8, 26, 14, 18, 11, 0, 0, 15, 2, 2, 3, 5, 3, 4, 6,
			10, 12, 12, 13, 90, 90, 0, 40, 103,97, 15, 0, 8, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 8,
			13, 0, 6, 0, 0, 13, 0, 11, 11, 0, 0, 0, 16, 17, 7, 1,
			15, 14, 42, 10, 3, 0, 0, 14, 7, 26, 40, 0, 5, 6, 38, 5,
			7, 2, 7, 21, 0, 7, 7, 16, 21, 12, 12, 16, 16, 10, 1, 1,
			1, 1, 1, 32, 10, 13, 6, 2, 21, 6, 13, 8, 6, 18, 5, 10,
			4, 20, 29, 0, 0, 0, 0, 0, 0, 2, 6, 6, 11, 7, 10, 33,
			13, 26, 6, 8, 0, 13, 9, 1, 7, 16, 17, 7, 0, 0, 7, 8,
			10, 7, 8, 24, 3, 8, 0, 7, 0, 7, 0, 7, 0, 0, 0, 0,
			1 ];

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
        // is invalid packet
		if raw.len() < 2 || (raw[0] >= 0xF0 && raw.len() < 3) {
			println!("D2GSReader::read(): invalid packet");
			return;
		}
		// or is plain packet
		if KNOWN_LENGTHS.contains(&(raw.len() as u16)) {
			self.packets.push_back(D2GSPacket{data: raw.to_vec()});

		}
		// else is compressed packet
		let mut decompressed_chunk = Vec::with_capacity(raw.len());
		let mut start: 		usize = 0;
		let mut nheader: 	usize = 0;
		let mut ndata: 		usize = 0;
		let mut end:		usize = 0;

		while (start + nheader + ndata) < raw.len() {

			ndata = self.get_chunk_params(&raw[start..], &mut nheader);
			end = start+nheader+ndata;
			huffman::decompress(&raw[start..end], &mut decompressed_chunk);
			self.packets.push_back(D2GSPacket{data: decompressed_chunk.clone()});
			start = end+1;
		}

		// After internal packet queue is filled, pop & handle all
		self.handle_all();
    }

	pub fn handle_all(&mut self) {
		while let Some(p) = self.packets.pop_front() {
			game_packet_dispatch(&p);
		}
	}


    fn get_chunk_params(&self, raw: &[u8], header_size: &mut usize) -> usize {
        if raw[0] < 0xF0 {
            *header_size = 1 as usize;
            return raw[0] as usize - 1;
        }
        *header_size = 2;
        // only bottom 3 nibbles due to header offset (-2)?
        return ( (( (raw[0] as u16) << 8) | raw[1] as u16) & 0xFFF ) as usize
    }

}
