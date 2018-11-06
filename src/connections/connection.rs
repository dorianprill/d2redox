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

struct ConnectionHook {
    iface:          &NetworkInterface,
    port:           u32,
    channel:        &Ethernet,
    protocol_state: ProtocolState
}

impl ConnectionHook {
        //pub fn new() {}
        //pub fn get_packet() -> pnet::packet::TcpPacket {}
        //pub fn build_packet() -> pnet::packet::TcpPacket {}
        //pub fn write_packet() -> bool {}
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
