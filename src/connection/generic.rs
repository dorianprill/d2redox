extern crate pnet;

//use self::pnet::packet::ethernet::Ethernet;
use self::pnet::datalink::{self, NetworkInterface};
use self::pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use self::pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use self::pnet::packet::ipv4::Ipv4Packet;
use self::pnet::packet::ipv6::Ipv6Packet;
use self::pnet::packet::tcp::TcpPacket;
use self::pnet::packet::udp::UdpPacket;
use self::pnet::packet::Packet;
use self::pnet::util::MacAddr;

//use std::env;
//use std::io::{self, Write};
use std::net::IpAddr;
use std::process;
use std::str;

//use std::collections::BinaryHeap;
//use connection::raw_packet::RawPacket;
use connection::d2gs::d2gs_reader::D2GSReader;

// There are three different connection involved:
// BNCS: the battle.net chat server
// Realm: (also called MCP)
// D2GS:  The diablo2 game server protocol

pub struct Connection {
    interface: NetworkInterface,
    initialized: bool,
    //protocol_state: ProtocolState,
    d2gs_reader: D2GSReader,
    // BinaryHeap as a PriorityQueue
    //packet_queue:   BinaryHeap<RawPacket<'a>>
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            interface: datalink::interfaces().pop().unwrap(),
            initialized: false,
            d2gs_reader: D2GSReader::new(),
        }
    }

    pub fn init(&mut self) {
        // Find the first network interface connected to the internet
        let interfaces = datalink::interfaces();
        let some_if: Option<NetworkInterface> = Some(
            interfaces
                .into_iter()
                .find(|ref ifx| ifx.is_up() && !ifx.is_loopback() && !ifx.ips.is_empty())
                .unwrap(),
        );

        //let interface: NetworkInterface;

        if some_if != None {
            self.interface = some_if.unwrap();
            println!("Identified network interface {}", self.interface);
        } else {
            println!("No active network adapter found, aborting...");
            process::exit(1);
        }

        self.initialized = true;
    }
    // thread function start_hook() ?
    //pub fn get_packet() -> pnet::packet::TcpPacket {}
    //pub fn build_packet() -> pnet::packet::TcpPacket {}
    // add_packet allows external modules to add packages for injecting
    //pub fn add_packet() -> bool {}
    pub fn listen(&mut self) {
        use self::pnet::datalink::Channel::Ethernet;
        let interface = self.interface.clone();
        if !self.initialized {
            println!("Connection: must init() before listen()");
            return;
        }
        // Create a channel to receive on
        let (_, mut rx_channel) = match datalink::channel(&self.interface, Default::default()) {
            Ok(Ethernet(tx, rx_channel)) => (tx, rx_channel),
            Ok(_) => panic!("{}", "unhandled channel type: {}"),
            Err(e) => panic!("unable to create channel: {}", e),
        };

        loop {
            let mut buf: [u8; 1600] = [0u8; 1600];
            let mut fake_ethernet_frame = MutableEthernetPacket::new(&mut buf[..]).unwrap();
            match rx_channel.next() {
                Ok(packet) => {
                    if cfg!(target_os = "macos")
                        && !interface.is_broadcast()
                        && interface.is_point_to_point()
                    {
                        // Maybe is TUN interface
                        let version = Ipv4Packet::new(&packet).unwrap().get_version();

                        fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                        fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                        if version == 4 {
                            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv4);
                            continue;
                        } else if version == 6 {
                            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv6);
                            continue;
                        }
                        fake_ethernet_frame.set_payload(&packet);
                        self.handle_ethernet_frame(&interface, &fake_ethernet_frame.to_immutable());
                    }
                    self.handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap());
                }
                Err(e) => panic!("unable to receive packet: {}", e),
            }
        }
    }

    fn handle_udp_packet(&mut self, _source: IpAddr, _destination: IpAddr, packet: &[u8]) {
        let udp = UdpPacket::new(packet);

        if let Some(udp) = udp {
            // filter packet by ports used by d2, will continue for both sent & received
            //if !PORTS.contains(&udp.get_destination()) && !PORTS.contains(&udp.get_source()) {
            //    return
            //}
            match udp.get_destination() {
                // game packet
                4000 => self.d2gs_reader.read(udp.payload()),
                // bncs/realm packet -> not implemented yet
                //6112 => println!("Received unhandled BNCS or Realm packet"),
                _ => {}
            }
            // println!(
            //         "UDP {}:{} > {}:{}  len={:03}  {:x?}  {:?}",
            //         source,
            //         udp.get_source(),
            //         destination,
            //         udp.get_destination(),
            //         udp.payload().len(),
            //         udp.payload(),
            //         String::from_utf8_lossy(udp.payload()).into_owned()
            // );
        } else {
            println!("Malformed UDP Packet");
        }
    }

    fn handle_tcp_packet(&mut self, _source: IpAddr, _destination: IpAddr, packet: &[u8]) {
        let tcp = TcpPacket::new(packet);
        if let Some(tcp) = tcp {
            // filter packet by ports used by d2, will continue for both sent & received
            //if !PORTS.contains(&tcp.get_destination()) {//&& !PORTS.contains(&tcp.get_source()) {
            //    return
            //}
            match tcp.get_destination() {
                // game packet
                4000 => self.d2gs_reader.read(tcp.payload()),
                // bncs/realm packet -> not implemented yet
                //6112 => println!("Received unhandled BNCS or Realm packet"),
                _ => {}
            }
            // println!(
            //     "TCP {}:{} > {}:{}  len={:03}  {:x?}  {:?}",
            //     source,
            //     tcp.get_source(),
            //     destination,
            //     tcp.get_destination(),
            //     tcp.payload().len(),
            //     tcp.payload(),
            //     String::from_utf8_lossy(tcp.payload()).into_owned()
            // );
        } else {
            println!("Malformed TCP Packet");
        }
    }

    fn handle_transport_protocol(
        &mut self,
        _interface_name: &str,
        source: IpAddr,
        destination: IpAddr,
        protocol: IpNextHeaderProtocol,
        packet: &[u8],
    ) {
        match protocol {
            IpNextHeaderProtocols::Udp => self.handle_udp_packet(source, destination, packet),
            IpNextHeaderProtocols::Tcp => self.handle_tcp_packet(source, destination, packet),
            _ => (),
            // _ => println!(
            //     "[{}]: Unhandled {} packet: {} > {}; protocol: {:?} length: {}",
            //     interface_name,
            //     match source {
            //         IpAddr::V4(..) => "IPv4",
            //         _ => "IPv6",
            //     },
            //     source,
            //     destination,
            //     protocol,
            //     packet.len()
            // ),
        }
    }

    fn handle_ipv4_packet(&mut self, interface_name: &str, ethernet: &EthernetPacket) {
        let header = Ipv4Packet::new(ethernet.payload());
        if let Some(header) = header {
            self.handle_transport_protocol(
                interface_name,
                IpAddr::V4(header.get_source()),
                IpAddr::V4(header.get_destination()),
                header.get_next_level_protocol(),
                header.payload(),
            );
        } else {
            println!("[{}]: Malformed IPv4 Packet", interface_name);
        }
    }

    fn handle_ipv6_packet(&mut self, interface_name: &str, ethernet: &EthernetPacket) {
        let header = Ipv6Packet::new(ethernet.payload());
        if let Some(header) = header {
            self.handle_transport_protocol(
                interface_name,
                IpAddr::V6(header.get_source()),
                IpAddr::V6(header.get_destination()),
                header.get_next_header(),
                header.payload(),
            );
        } else {
            println!("[{}]: Malformed IPv6 Packet", interface_name);
        }
    }

    fn handle_ethernet_frame(&mut self, interface: &NetworkInterface, ethernet: &EthernetPacket) {
        let interface_name = &interface.name[..];
        match ethernet.get_ethertype() {
                EtherTypes::Ipv4 => self.handle_ipv4_packet(interface_name, ethernet),
                EtherTypes::Ipv6 => self.handle_ipv6_packet(interface_name, ethernet),
                _ => ()
                    // TODO make debug only print
                    // println!(
                    // "[{}]: Unknown packet: {} > {}; ethertype: {:?} length: {}",
                    // interface_name,
                    // ethernet.get_source(),
                    // ethernet.get_destination(),
                    // ethernet.get_ethertype(),
                    // ethernet.packet().len()
                //),
            }
    }
}
