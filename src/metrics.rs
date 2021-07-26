use lazy_static::lazy_static;
use prometheus::{self, IntCounterVec, IntCounter, IntGauge};
use prometheus::{register_int_counter_vec, register_int_counter, register_int_gauge, Opts};

// Prometheus Metrics

const NAMESPACE: &str = "bitcoindobserver";

const SUBSYSTEM_RUNTIME: &str = "runtime";
const SUBSYSTEM_P2P: &str = "p2p";
const SUBSYSTEM_VALIDATION: &str = "validation";

pub const LABEL_P2P_MSG_TYPE: &str = "msg_type";
pub const LABEL_P2P_CONNECTION_TYPE: &str = "connection_type";

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
}
