use std::io;

const IPV4: u16 = 0x0800;
const TCP: u8 = 0x06;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to cr");
    let mut buf = [0u8; 1504];


    loop {
        // listen for packages
        let nbytes = nic.recv(&mut buf[..])?;

        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        // skip if no ipv4
        if eth_proto != IPV4 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(packet) => {
                let proto = packet.protocol();

                // skip if no tcp
                if proto != TCP {
                    continue;
                }

                let src = packet.source_addr();
                let dest = packet.destination_addr();
                let payload_len = packet.payload_len();

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + packet.slice().len()..]) {
                    Ok(p) => {
                        let port = p.destination_port();
                        eprintln!("RECV TCP {} -> {}:{} | {} bytes", src, dest, port, p.slice().len());
                    },
                    Err(e) => {
                        eprintln!("IGNORING PACKET WITH ERR {:?}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("IGNORING PACKET WITH ERR {:?}", e);
            }
        }

    }
}
