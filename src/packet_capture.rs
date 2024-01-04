use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::{ipv4::Ipv4Packet, udp::UdpPacket, Packet};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::env;

pub fn run_packet_capture(domains: Arc<Mutex<Vec<String>>>) {
    // Introduce a delay before attempting to capture packets (adjust the duration as needed)
    thread::sleep(Duration::from_secs(5));

    // Get a list of available network interfaces
    let interfaces = datalink::interfaces();
    for iface in &interfaces {
        println!("Network Interface Name: {}", iface.name);
    }

    // Choose the network interface you want to capture packets on (e.g., en0)
    let interface_name = env::var("NETWORK_INTERFACE").unwrap();

    let interface = interfaces
        .iter()
        .find(|iface| iface.name == interface_name)
        .expect("Interface not found");

    // Create a new channel to capture Ethernet packets on the selected interface
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create channel: {}", e),
    };

    // Start packet capture loop
    loop {
        match rx.next() {
            Ok(packet) => {
                // Parse Ethernet packet
                if let Some(eth) = EthernetPacket::new(packet) {
                    if eth.get_ethertype() == EtherTypes::Ipv4 {
                        // Parse IPv4 packet
                        if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                            // Check if it's a UDP packet (DNS typically uses UDP)
                            if ipv4.get_next_level_protocol()
                                == pnet::packet::ip::IpNextHeaderProtocols::Udp
                            {
                                // Parse UDP packet
                                if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                    // DNS typically uses port 53
                                    if udp.get_destination() == 53 {
                                        // Extract domain name from DNS request
                                        let dns_payload = udp.payload();
                                        if dns_payload.len() >= 5 {
                                            let qdcount = (dns_payload[4] as usize) << 8
                                                | (dns_payload[5] as usize);
                                            let mut offset = 12; // DNS header size

                                            for _ in 0..qdcount {
                                                // Parse domain name labels
                                                let mut domain_name = String::new();
                                                let mut label_len = dns_payload[offset] as usize;
                                                offset += 1;

                                                while label_len > 0 {
                                                    if domain_name.len() > 0 {
                                                        domain_name.push('.');
                                                    }

                                                    domain_name += str::from_utf8(
                                                        &dns_payload[offset..offset + label_len],
                                                    )
                                                    .unwrap_or("<invalid>");

                                                    offset += label_len;
                                                    label_len = dns_payload[offset] as usize;
                                                    offset += 1;

                                                    // Add the captured domain to the shared data structure
                                                    let mut captured_domains =
                                                        domains.lock().unwrap();
                                                    captured_domains.push(domain_name.clone());

                                                    // Debug: Print the captured domain
                                                    // println!("Captured Domain: {}", domain_name);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error while capturing packet: {}", e),
        }
    }
}
