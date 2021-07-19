use bcc::perf_event::PerfMapBuilder;
use bcc::{BPFBuilder, USDTContext};
use std::{env, fmt, ptr};

// Tor v3 addresses are 62 chars + 6 chars for the port (':12345').
const MAX_PEER_ADDR_LENGTH: usize = 62 + 6;
const MAX_PEER_CONN_TYPE_LENGTH: usize = 20;
const MAX_MSG_TYPE_LENGTH: usize = 20;

/// Represents an inbound or outbound P2P message.
#[repr(C)]
struct p2p_msg {
    peer_id: u64,
    peer_addr: [u8; MAX_PEER_ADDR_LENGTH],
    peer_conn_type: [u8; MAX_PEER_CONN_TYPE_LENGTH],
    msg_type: [u8; MAX_MSG_TYPE_LENGTH],
    msg_size: u64,
}

impl fmt::Display for p2p_msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "peer {} ({}, {}): {} with {} bytes",
            self.peer_id,
            String::from_utf8_lossy(&self.peer_addr.split(|c| *c == 0x00u8).next().unwrap()),
            String::from_utf8_lossy(&self.peer_conn_type.split(|c| *c == 0x00u8).next().unwrap()),
            String::from_utf8_lossy(&self.msg_type.split(|c| *c == 0x00u8).next().unwrap()),
            self.msg_size,
        )
    }
}

fn main() {
    let bitcoind_path = env::args().nth(1).expect("No bitcoind path provided.");

    let mut usdt_ctx = USDTContext::from_binary_path(bitcoind_path).unwrap();
    usdt_ctx
        .enable_probe("net:inbound_message", "trace_inbound_message")
        .unwrap();
    usdt_ctx
        .enable_probe("net:outbound_message", "trace_outbound_message")
        .unwrap();

    let code = include_str!("../ebpf-programs/p2p-in-and-outbound.c");
    let bpf = BPFBuilder::new(code)
        .unwrap()
        .add_usdt_context(usdt_ctx)
        .unwrap()
        .build()
        .unwrap();

    let table_inbound_messages = bpf.table("inbound_messages").unwrap();
    let table_outbound_messages = bpf.table("outbound_messages").unwrap();

    let mut perf_map_inbound_msg =
        PerfMapBuilder::new(table_inbound_messages, callback_inbound_message)
            .build()
            .unwrap();
    let mut perf_map_outbound_msg =
        PerfMapBuilder::new(table_outbound_messages, callback_outbound_message)
            .build()
            .unwrap();

    loop {
        perf_map_inbound_msg.poll(200);
        perf_map_outbound_msg.poll(200);
    }
}

fn callback_inbound_message() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        println!("inbound message from {}", parse_p2p_message(x));
    })
}

fn callback_outbound_message() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        println!("outbound message from {}", parse_p2p_message(x));
    })
}

fn parse_p2p_message(x: &[u8]) -> p2p_msg {
    unsafe { ptr::read_unaligned(x.as_ptr() as *const p2p_msg) }
}
