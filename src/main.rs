use bcc::perf_event::PerfMapBuilder;
use bcc::{BPFBuilder, USDTContext};
use std::collections::HashMap;
use std::env;
use std::time;

mod metrics;
mod metricserver;
mod types;

use types::{BlockConnected, P2PMessage, UTXOCacheEvent, UTXOCacheFlush, SyncEvent};

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
    usdt_ctx
        .enable_probe("utxocache:flush", "trace_utxocache_flush")
        .unwrap();
    /*
    usdt_ctx
        .enable_probe("sync:enter", "trace_sync_enter")
        .unwrap();
    usdt_ctx
        .enable_probe("sync:locked", "trace_sync_locked")
        .unwrap();
    usdt_ctx
        .enable_probe("sync:try_locked", "trace_sync_try_locked")
        .unwrap();
*/
    usdt_ctx
        .enable_probe("sync:unlock", "trace_sync_unlocked")
        .unwrap();
    let code = concat!(
        "#include <uapi/linux/ptrace.h>",
        "\n\n",
        include_str!("../ebpf-programs/p2p_in_and_outbound.c"),
        include_str!("../ebpf-programs/validation_block_connected.c"),
        include_str!("../ebpf-programs/utxo_set_cache_changes.c"),
        include_str!("../ebpf-programs/sync_locks.c"),
        include_str!("../ebpf-programs/utxo_set_cache_flushes.c"),
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
    let table_utxocache_flushes = bpf.table("perf_utxocache_flushes").unwrap();
    let table_sync_enter_events = bpf.table("sync_enter").unwrap();
    let table_sync_locked_events = bpf.table("sync_locked").unwrap();
    let table_sync_try_locked_events = bpf.table("sync_try_locked").unwrap();
    let table_sync_unlocked_events = bpf.table("sync_unlocked").unwrap();

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
    let mut perf_map_utxocache_flushes =
        PerfMapBuilder::new(table_utxocache_flushes, callback_utxocache_flush)
            .build()
            .unwrap();

    let mut perf_map_sync_enter_events =
        PerfMapBuilder::new(table_sync_enter_events, callback_sync_enter)
            .build()
            .unwrap();
    let mut perf_map_sync_locked_events =
        PerfMapBuilder::new(table_sync_locked_events, callback_sync_locked)
            .build()
            .unwrap();
    let mut perf_map_sync_try_locked_events =
        PerfMapBuilder::new(table_sync_try_locked_events, callback_sync_try_locked)
            .build()
            .unwrap();
    let mut perf_map_sync_unlocked_events =
        PerfMapBuilder::new(table_sync_unlocked_events, callback_sync_unlocked)
            .build()
            .unwrap();

    metricserver::start(&metricserver_address).unwrap();

    log::info!(target: LOG_TARGET, "Started bitcoind-observer.");

    loop {
        perf_map_inbound_msg.poll(1);
        perf_map_outbound_msg.poll(1);
        perf_map_block_connected.poll(1);
        perf_map_utxocache_events.poll(1);
        perf_map_utxocache_flushes.poll(1);
        perf_map_sync_enter_events.poll(1);
        perf_map_sync_locked_events.poll(1);
        perf_map_sync_try_locked_events.poll(1);
        perf_map_sync_unlocked_events.poll(1);
    }
}

fn callback_sync_enter() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let event = SyncEvent::from_bytes(x);
        println!("SyncEnter: {}", event);
    })
}

fn callback_sync_unlocked() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let event = SyncEvent::from_bytes(x);
        println!("SyncUnlocked: {}", event);
    })
}

fn callback_sync_locked() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let event = SyncEvent::from_bytes(x);
        println!("SyncLocked: {}", event);
    })
}

fn callback_sync_try_locked() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let event = SyncEvent::from_bytes(x);
        println!("SyncTryLocked: {}", event);
    })
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

fn callback_utxocache_flush() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let flush = UTXOCacheFlush::from_bytes(x);
        let mut labels = HashMap::<&str, &str>::new();
        labels.insert(metrics::LABEL_UTXOCACHE_FLUSH_MODE, flush.flush_mode());
        labels.insert(metrics::LABEL_UTXOCACHE_FLUSH_FORPRUNE, flush.flush_for_prune());
        metrics::UTXOCACHE_FLUSH.with(&labels).inc();
        metrics::UTXOCACHE_FLUSH_DURATION.with(&labels).inc_by(flush.duration);
        metrics::UTXOCACHE_FLUSH_COINS_COUNT.with(&labels).inc_by(flush.coins_count);
        metrics::UTXOCACHE_FLUSH_COINS_MEMUSAGE.with(&labels).inc_by(flush.coins_memusage);
    })
}
