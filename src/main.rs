// Dorian Prill <drn@tuta.io>
//
// Entry point for d2re (Diablo II Reverse[d] Engine).
// Sniffs all network packets sent by:
//  1) Diablo II Game Server (D2GS)
//  2) Battle.net Chat Server (BNCS)
//  3) Realm Server for logon (MCP)
// Then hands them over to the appropriate handling procedures.
// WIP currently, ultimate goal would be enabling clientless botting

#![allow(dead_code)]

mod client;
mod connection;
mod engine;
mod game;

use connection::generic::Connection;


fn main() {
    let mut connection = Connection::new();
    connection.init();
    connection.listen();
}
