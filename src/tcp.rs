enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}
/// State of the Send Sequence Space (RFC 793 S3.2 F4)
struct SendSequenceSpace {
    /// send unacknowledged
    una: usize,
    /// send next
    nxt: usize,
    /// send window
    wnd: usize,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update,
    wl1: usize,
    /// segment acknowledgement number used for last window update,
    wl2: usize,
    /// initial send sequence number
    iss: usize
}

/// State of the Receive Sequence Space (RFC 793 S3.2 F5)
struct RecvSequenceSpace {
    /// receive next
    nxt: usize,
    /// receive window
    wnd: usize,
    /// receive urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: usize
}
pub struct Connection {
    state: State
}

impl Default for Connection {
    fn default() -> Self {
        Connection {state: State::Listen}
    }
}

impl Connection {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice,
        tcp_header: etherparse::TcpHeaderSlice,
        data: &'a [u8],
    ) -> std::io::Result<usize> {
        let mut buf = [0u8; 1500];

        eprintln!(
                "RECV TCP {}:{} -> {}:{} | {} bytes",
        ip_header.source_addr(),
        tcp_header.source_port(),
        ip_header.destination_addr(),
        tcp_header.destination_port(),
        data.len()
        );

        match (*self).state {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcp_header.syn() {
                    // packet unexpectedly not SYN
                    return Ok(0);
                }
                // send syn ack
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    unimplemented!(),
                    unimplemented!(),
                );
                syn_ack.syn = true;
                syn_ack.ack = true;

                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::IpTrafficClass::Tcp,
                [
                    ip_header.destination()[0],
                    ip_header.destination()[1],
                    ip_header.destination()[2],
                    ip_header.destination()[3],
                ],
                [
                    ip_header.source()[0],
                    ip_header.source()[1],
                    ip_header.source()[2],
                    ip_header.source()[3],
                    ],
                );

                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten);
                    syn_ack.write(&mut unwritten);
                    unwritten.len()
                };
                nic.send(&buf[..unwritten])
            }
            State::SynRcvd => unimplemented!(),
            State::Estab => unimplemented!(),
        }
    }
}
