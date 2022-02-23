// Entry point for d2re (Diablo II Reverse[d] Engine).
// Sniffs all network packets sent by:
//  1) Diablo II Game Server (D2GS)
//  2) Battle.net Chat Server (BNCS)
//  3) Realm Server for logon (MCP)
// Then hands them over to the appropriate handling procedures.
// WIP currently, ultimate goal would be enabling clientless botting

#![allow(dead_code)]

use libd2r::client::client::Client;

fn main() {
    let mut client = Client::new();
    client.start()
}
