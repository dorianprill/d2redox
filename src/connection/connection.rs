#![feature(ptr_internals, allocator_api)]
use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
use std::mem;
use std::ptr::{drop_in_place, NonNull, Unique};
use std::env;
use std::io::{self, Write};
use std::net::IpAddr;
use std::process;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
// packet types
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
// other
use pnet::util::MacAddr;

use std::collections::BinaryHeap

use raw_packet::RawPacket;
use d2gs::D2GSPacket;

// There are three different connection involved:
// BNCS: the battle.net chat server
// Realm: (also called MCP)
// D2GS:  The diablo2 game server protocol
enum ProtocolState {
    Undefined
    BNCS,
    Realm,
    D2GS
}

struct Connection {
    iface:          &NetworkInterface,
    //port:           u32,
    channel:        &Ethernet,
    protocol_state: ProtocolState,
	// BinaryHeap is a PriorityQueue
	// -> raw packets implement Ord trait
	packet_queue:   BinaryHeap<RawPacket>
}

impl Connection {
        pub fn new(&NetworkInterface) {
			return true
		}
		// thread function start_hook() ?
        //pub fn get_packet() -> pnet::packet::TcpPacket {}
        //pub fn build_packet() -> pnet::packet::TcpPacket {}
		// add_packet allows external modules to add packages for injecting
        //pub fn add_packet() -> bool {}
		pub fn listen(iface: &NetworkInterface) -> bool {
			return true;
		}

}

impl Drop for Connection {
    fn drop(&mut self) {
        // close all connections of members
        // ...
        // then drop self
        drop_in_place(self.ptr.as_ptr());
        let c: NonNull<T> = self.ptr.into();
        Global.dealloc(c.cast(), Layout::new::<T>())
    }
}
