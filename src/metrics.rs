use lazy_static::lazy_static;
use prometheus::{self, IntCounter, IntCounterVec, IntGauge};
use prometheus::{register_int_counter, register_int_counter_vec, register_int_gauge, Opts};

// Prometheus Metrics

const NAMESPACE: &str = "bitcoindobserver";

const SUBSYSTEM_RUNTIME: &str = "runtime";
const SUBSYSTEM_P2P: &str = "p2p";
const SUBSYSTEM_VALIDATION: &str = "validation";
const SUBSYSTEM_UTXOCACHE: &str = "utxocache";
const SUBSYSTEM_MEMPOOL: &str = "mempool";

pub const LABEL_P2P_MSG_TYPE: &str = "msg_type";
pub const LABEL_P2P_CONNECTION_TYPE: &str = "connection_type";

pub const LABEL_UTXOCACHE_FLUSH_MODE: &str = "flush_mode";
pub const LABEL_UTXOCACHE_FLUSH_FORPRUNE: &str = "for_prune";
pub const LABEL_MEMPOOL_REASON: &str = "reason";

lazy_static! {

    // -------------------- Runtime

    /// UNIX epoch timestamp of bitcoind-observer start. Can be used to alert on
    /// bitcoind-observer restarts.
    pub static ref RUNTIME_START_TIMESTAMP: IntGauge =
        register_int_gauge!(
            Opts::new("start_timestamp", "UNIX epoch timestamp of bitcoind-observer start")
                .namespace(NAMESPACE)
                .subsystem(SUBSYSTEM_RUNTIME)
        ).unwrap();

    // -------------------- P2P

    /// Number of inbound P2P network messages received.
    pub static ref P2P_MESSAGE_INBOUND_COUNT: IntCounterVec =
        register_int_counter_vec!(
            Opts::new("message_inbound_count", "Number of inbound P2P network messages received.")
                .namespace(NAMESPACE)
                .subsystem(SUBSYSTEM_P2P),
            &[LABEL_P2P_MSG_TYPE, LABEL_P2P_CONNECTION_TYPE]
        ).unwrap();

    /// Number of outbound P2P network messages send.
    pub static ref P2P_MESSAGE_OUTBOUND_COUNT: IntCounterVec =
        register_int_counter_vec!(
            Opts::new("message_outbound_count", "Number of outbound P2P network messages send.")
                .namespace(NAMESPACE)
                .subsystem(SUBSYSTEM_P2P),
            &[LABEL_P2P_MSG_TYPE, LABEL_P2P_CONNECTION_TYPE]
        ).unwrap();

    /// Number of inbound P2P network messages bytes received.
    pub static ref P2P_MESSAGE_INBOUND_BYTE: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("message_inbound_bytes", "Number of inbound P2P network messages bytes received.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_P2P),
        &[LABEL_P2P_MSG_TYPE, LABEL_P2P_CONNECTION_TYPE]
    ).unwrap();

    /// Number of outbound P2P network messages bytes send.
    pub static ref P2P_MESSAGE_OUTBOUND_BYTE: IntCounterVec =
        register_int_counter_vec!(
            Opts::new("message_outbound_bytes", "Number of outbound P2P network messages bytes send..")
                .namespace(NAMESPACE)
                .subsystem(SUBSYSTEM_P2P),
            &[LABEL_P2P_MSG_TYPE, LABEL_P2P_CONNECTION_TYPE]
        ).unwrap();

    // -------------------- VALIDATION

    /// Last block height connected
    pub static ref VALIDATION_BLOCK_CONNECTED_HEIGHT_LAST: IntGauge =
    register_int_gauge!(
        Opts::new("block_connected_height_last", "Last block height connected.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_VALIDATION)
    ).unwrap();

    /// Number of connected blocks
    pub static ref VALIDATION_BLOCK_CONNECTED_COUNT: IntCounter =
    register_int_counter!(
        Opts::new("block_connected_count", "Number of connected blocks.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_VALIDATION)
    ).unwrap();

    /// Number of transactions in the connected blocks
    pub static ref VALIDATION_BLOCK_CONNECTED_TRANSACTION_COUNT: IntCounter =
    register_int_counter!(
        Opts::new("block_connected_transaction_count", "Number of transactions in the connected blocks.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_VALIDATION)
    ).unwrap();

    /// Number of inputs in the connected blocks
    pub static ref VALIDATION_BLOCK_CONNECTED_INPUT_COUNT: IntCounter =
    register_int_counter!(
        Opts::new("block_connected_input_count", "Number of inputs in the connected blocks.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_VALIDATION)
    ).unwrap();

    /// Number of sigops in the connected blocks
    pub static ref VALIDATION_BLOCK_CONNECTED_SIGOP_COUNT: IntCounter =
    register_int_counter!(
        Opts::new("block_connected_sigops_count", "Number of sigops in the connected blocks.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_VALIDATION)
    ).unwrap();

    /// Time block connection took in microseconds (µs)
    pub static ref VALIDATION_BLOCK_CONNECTED_TIMING: IntCounter =
    register_int_counter!(
        Opts::new("block_connected_timing", "Time block connection took in microseconds (µs).")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_VALIDATION)
    ).unwrap();

    // -------------------- UTXO Cache

    /// Additions to the UTXO set cache.
    pub static ref UTXOCACHE_ADD: IntCounter =
    register_int_counter!(
        Opts::new("add", "Additions to the UTXO set cache.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_UTXOCACHE)
    ).unwrap();

    /// Spents from the UTXO set cache.
    pub static ref UTXOCACHE_SPENT: IntCounter =
    register_int_counter!(
        Opts::new("spent", "Spents from the UTXO set cache.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_UTXOCACHE)
    ).unwrap();

    /// Uncaches from the UTXO set cache.
    pub static ref UTXOCACHE_UNCACHE: IntCounter =
    register_int_counter!(
        Opts::new("uncache", "Uncaches from the UTXO set cache.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_UTXOCACHE)
    ).unwrap();

    /// UTXO set cache flush.
    pub static ref UTXOCACHE_FLUSH: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("flush", "UTXO set cache flush.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_UTXOCACHE),
            &[LABEL_UTXOCACHE_FLUSH_MODE, LABEL_UTXOCACHE_FLUSH_FORPRUNE]
    ).unwrap();

    /// Total UTXO set cache flush duration.
    pub static ref UTXOCACHE_FLUSH_DURATION: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("flush_duration", "Total UTXO set cache flush duration.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_UTXOCACHE),
            &[LABEL_UTXOCACHE_FLUSH_MODE, LABEL_UTXOCACHE_FLUSH_FORPRUNE]
    ).unwrap();

    /// Total UTXO set cache coins flushed.
    pub static ref UTXOCACHE_FLUSH_COINS_COUNT: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("flush_coins_count", "Total UTXO set cache coins flushed.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_UTXOCACHE),
            &[LABEL_UTXOCACHE_FLUSH_MODE, LABEL_UTXOCACHE_FLUSH_FORPRUNE]
    ).unwrap();

    /// Total UTXO set cache memory flushed.
    pub static ref UTXOCACHE_FLUSH_COINS_MEMUSAGE: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("flush_coins_memusage", "Total UTXO set cache memory flushed.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_UTXOCACHE),
            &[LABEL_UTXOCACHE_FLUSH_MODE, LABEL_UTXOCACHE_FLUSH_FORPRUNE]
    ).unwrap();

    // -------------------- Mempool

    /// Transactions added to the mempool
    pub static ref MEMPOOL_ADDED: IntCounter =
    register_int_counter!(
        Opts::new("added", "Transactions added to the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL)
    ).unwrap();

    /// Fees added to the mempool
    pub static ref MEMPOOL_FEE_ADDED: IntCounter =
    register_int_counter!(
        Opts::new("fees_added", "Fees added to the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL)
    ).unwrap();

    /// Vsize added to the mempool
    pub static ref MEMPOOL_VSIZE_ADDED: IntCounter =
    register_int_counter!(
        Opts::new("vsize_added", "VSize added to the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL)
    ).unwrap();

    /// Transactions removed from the mempool
    pub static ref MEMPOOL_REMOVED: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("removed", "Transactions removed from the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL),
            &[LABEL_MEMPOOL_REASON]
    ).unwrap();

    /// Fees removed from the mempool
    pub static ref MEMPOOL_FEE_REMOVED: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("fees_removed", "Fees removed from the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL),
            &[LABEL_MEMPOOL_REASON]
    ).unwrap();

    /// Vsize removed from the mempool
    pub static ref MEMPOOL_VSIZE_REMOVED: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("vsize_removed", "VSize removed from the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL),
            &[LABEL_MEMPOOL_REASON]
    ).unwrap();

    /// Transactions rejected from entering the mempool
    pub static ref MEMPOOL_REJECTED: IntCounterVec =
    register_int_counter_vec!(
        Opts::new("rejected", "Transactions rejected from entering the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL),
            &[LABEL_MEMPOOL_REASON]
    ).unwrap();

    /// Transactions replaced in the mempool
    pub static ref MEMPOOL_REPLACED: IntCounter =
    register_int_counter!(
        Opts::new("replaced", "Transactions replaced in the mempool.")
            .namespace(NAMESPACE)
            .subsystem(SUBSYSTEM_MEMPOOL)
    ).unwrap();

}
