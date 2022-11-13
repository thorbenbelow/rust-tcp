use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to cr");
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        // skip if no ipv4
        if eth_proto != 0x0800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(packet) => {
                let src = packet.source_addr();
                let dest = packet.destination_addr();
                let proto = packet.protocol();
                let payload_len = packet.payload_len();

                eprintln!("{} -> {} : {} bytes of protocol {}", src, dest, payload_len, proto);
            },
            Err(e) => {
                eprintln!("IGNORING PACKET WITH ERR {:?}", e);
            }
        }

    }
}
