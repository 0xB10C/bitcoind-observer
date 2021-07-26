use bcc::perf_event::PerfMapBuilder;
use bcc::{BPFBuilder, USDTContext};
use std::collections::HashMap;
use std::env;
use std::time;

mod metrics;
mod metricserver;
mod types;

use types::P2PMessage;

use simple_logger::SimpleLogger;

const LOG_TARGET: &str = "main";

fn main() {
    let bitcoind_path = env::args().nth(1).expect("No bitcoind path provided.");
    let metricserver_address = env::args()
        .nth(2)
        .expect("No metric server address to bind on provided (.e.g. 'localhost:8282').");

    SimpleLogger::new()
        .init()
        .expect("Could not setup logging.");

    log::info!(
        target: LOG_TARGET,
        "Starting bitcoind-observer using {} ...",
        bitcoind_path,
    );

    metrics::RUNTIME_START_TIMESTAMP.set(
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    );

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

    metricserver::start(&metricserver_address).unwrap();

    log::info!(target: LOG_TARGET, "Started bitcoind-observer.");

    loop {
        perf_map_inbound_msg.poll(200);
        perf_map_outbound_msg.poll(200);
    }
}

fn callback_inbound_message() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let inbound_msg = P2PMessage::from_bytes(x);
        let msg_type = inbound_msg.get_msg_type();
        let conn_type = inbound_msg.get_peer_conn_type();
        let mut labels = HashMap::<&str, &str>::new();
        labels.insert(metrics::LABEL_P2P_MSG_TYPE, &msg_type);
        labels.insert(metrics::LABEL_P2P_CONNECTION_TYPE, &conn_type);
        metrics::P2P_MESSAGE_INBOUND_COUNT.with(&labels).inc();
        metrics::P2P_MESSAGE_INBOUND_BYTE
            .with(&labels)
            .inc_by(inbound_msg.msg_size);
    })
}

fn callback_outbound_message() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let outbound_msg = P2PMessage::from_bytes(x);
        let msg_type = outbound_msg.get_msg_type();
        let conn_type = outbound_msg.get_peer_conn_type();
        let mut labels = HashMap::<&str, &str>::new();
        labels.insert(metrics::LABEL_P2P_MSG_TYPE, &msg_type);
        labels.insert(metrics::LABEL_P2P_CONNECTION_TYPE, &conn_type);
        metrics::P2P_MESSAGE_OUTBOUND_COUNT.with(&labels).inc();
        metrics::P2P_MESSAGE_OUTBOUND_BYTE
            .with(&labels)
            .inc_by(outbound_msg.msg_size);
    })
}
