use bcc::perf_event::PerfMapBuilder;
use bcc::{BPFBuilder, USDTContext};
use std::collections::HashMap;
use std::env;
use std::time;

mod metrics;
mod metricserver;
mod types;

use types::{BlockConnected, P2PMessage, UTXOCacheEvent};

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
    usdt_ctx
        .enable_probe("validation:block_connected", "trace_block_connected")
        .unwrap();
    usdt_ctx
        .enable_probe("utxocache:add", "trace_utxocache_add")
        .unwrap();
    usdt_ctx
        .enable_probe("utxocache:spent", "trace_utxocache_spent")
        .unwrap();
    usdt_ctx
        .enable_probe("utxocache:uncache", "trace_utxocache_uncache")
        .unwrap();

    let code = concat!(
        "#include <uapi/linux/ptrace.h>",
        "\n\n",
        include_str!("../ebpf-programs/p2p_in_and_outbound.c"),
        include_str!("../ebpf-programs/validation_block_connected.c"),
        include_str!("../ebpf-programs/utxo_set_cache_changes.c"),
    );
    let bpf = BPFBuilder::new(code)
        .unwrap()
        .add_usdt_context(usdt_ctx)
        .unwrap()
        .build()
        .unwrap();

    let table_inbound_messages = bpf.table("inbound_messages").unwrap();
    let table_outbound_messages = bpf.table("outbound_messages").unwrap();
    let table_block_connected = bpf.table("perf_block_connected").unwrap();
    let table_utxocache_events = bpf.table("perf_utxocache_events").unwrap();

    let mut perf_map_inbound_msg =
        PerfMapBuilder::new(table_inbound_messages, callback_inbound_message)
            .build()
            .unwrap();
    let mut perf_map_outbound_msg =
        PerfMapBuilder::new(table_outbound_messages, callback_outbound_message)
            .build()
            .unwrap();
    let mut perf_map_block_connected =
        PerfMapBuilder::new(table_block_connected, callback_block_connected)
            .build()
            .unwrap();
    let mut perf_map_utxocache_events =
        PerfMapBuilder::new(table_utxocache_events, callback_utxocache_event)
            .build()
            .unwrap();

    metricserver::start(&metricserver_address).unwrap();

    log::info!(target: LOG_TARGET, "Started bitcoind-observer.");

    loop {
        perf_map_inbound_msg.poll(1);
        perf_map_outbound_msg.poll(1);
        perf_map_block_connected.poll(1);
        perf_map_utxocache_events.poll(1);
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

fn callback_block_connected() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let block_connected = BlockConnected::from_bytes(x);
        metrics::VALIDATION_BLOCK_CONNECTED_HEIGHT_LAST.set(block_connected.height as i64);
        metrics::VALIDATION_BLOCK_CONNECTED_COUNT.inc();
        metrics::VALIDATION_BLOCK_CONNECTED_TRANSACTION_COUNT.inc_by(block_connected.transactions);
        metrics::VALIDATION_BLOCK_CONNECTED_INPUT_COUNT.inc_by(block_connected.inputs as u64);
        metrics::VALIDATION_BLOCK_CONNECTED_SIGOP_COUNT.inc_by(block_connected.sigops);
        metrics::VALIDATION_BLOCK_CONNECTED_TIMING.inc_by(block_connected.connection_time);
    })
}

fn callback_utxocache_event() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let event = UTXOCacheEvent::from_bytes(x);
        match event.event {
            types::UTXOCACHE_ADD => metrics::UTXOCACHE_ADD.inc(),
            types::UTXOCACHE_SPENT => metrics::UTXOCACHE_SPENT.inc(),
            types::UTXOCACHE_UNCACHE => metrics::UTXOCACHE_UNCACHE.inc(),
            _ => log::info!(
                target: LOG_TARGET,
                "UTXO cache event callback: unknown event {:?}",
                event.event
            ),
        }
    })
}
