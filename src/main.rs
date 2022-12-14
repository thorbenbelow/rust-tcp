use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Eq, PartialEq, Hash)]
struct Quad {
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();

    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to cr");
    let mut buf = [0u8; 1504];

    loop {
        // listen for packages
        let nbytes = nic.recv(&mut buf[..])?;

        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        // skip if no ipv4
        if eth_proto != etherparse::EtherType::Ipv4 as u16 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_header) => {
                // skip if no tcp
                if ip_header.protocol() != etherparse::IpTrafficClass::Tcp as u8 {
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip_header.slice().len()..nbytes]) {
                    Ok(tcp_header) => {
                        let data_idx = 4 + ip_header.slice().len() + tcp_header.slice().len();

                        let src = (ip_header.source_addr(), tcp_header.source_port());
                        let dest = (ip_header.destination_addr(), tcp_header.destination_port());

                        connections
                            .entry(Quad { src, dest })
                            .or_default()
                            .on_packet(&mut nic, ip_header, tcp_header, &buf[data_idx..nbytes])?;
                    }
                    Err(e) => {
                        eprintln!("IGNORING PACKET WITH ERR {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("IGNORING PACKET WITH ERR {:?}", e);
            }
        }
    }
}
