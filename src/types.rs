use std::{fmt, ptr};

// Tor v3 addresses are 62 chars + 6 chars for the port (':12345').
const MAX_PEER_ADDR_LENGTH: usize = 62 + 6;
const MAX_PEER_CONN_TYPE_LENGTH: usize = 20;
const MAX_MSG_TYPE_LENGTH: usize = 20;

/// Represents an inbound or outbound P2P message.
#[repr(C)]
pub struct P2PMessage {
    pub peer_id: u64,
    pub peer_addr: [u8; MAX_PEER_ADDR_LENGTH],
    pub peer_conn_type: [u8; MAX_PEER_CONN_TYPE_LENGTH],
    pub msg_type: [u8; MAX_MSG_TYPE_LENGTH],
    pub msg_size: u64,
}

impl P2PMessage {
    pub fn from_bytes(x: &[u8]) -> P2PMessage {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const P2PMessage) }
    }

    pub fn get_peer_addr(&self) -> String {
        String::from_utf8_lossy(&self.peer_addr.split(|c| *c == 0x00u8).next().unwrap())
            .into_owned()
    }

    pub fn get_peer_conn_type(&self) -> String {
        String::from_utf8_lossy(&self.peer_conn_type.split(|c| *c == 0x00u8).next().unwrap())
            .into_owned()
    }

    pub fn get_msg_type(&self) -> String {
        String::from_utf8_lossy(&self.msg_type.split(|c| *c == 0x00u8).next().unwrap()).into_owned()
    }
}

impl fmt::Display for P2PMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "peer {} ({}, {}): {} with {} bytes",
            self.peer_id,
            self.get_peer_addr(),
            self.get_peer_conn_type(),
            self.get_msg_type(),
            self.msg_size,
        )
    }
}

/// Represents a connected block.
#[repr(C)]
pub struct BlockConnected {
    pub height: i32,
    pub transactions: u64,
    pub inputs: i32,
    pub sigops: u64,
    pub connection_time: u64,
}

impl BlockConnected {
    pub fn from_bytes(x: &[u8]) -> BlockConnected {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const BlockConnected) }
    }
}

impl fmt::Display for BlockConnected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "connected height={} tx={}, ins={}, sigops={} time={}µs",
            self.height, self.transactions, self.inputs, self.sigops, self.connection_time,
        )
    }
}

pub const UTXOCACHE_ADD: u8 = 0;
pub const UTXOCACHE_SPENT: u8 = 1;
pub const UTXOCACHE_UNCACHE: u8 = 2;

/// Represents a utxocache event (utxocache:{add, spent, uncache} tracepoints).
#[repr(C)]
pub struct UTXOCacheEvent {
    pub event: u8,
}

impl UTXOCacheEvent {
    pub fn from_bytes(x: &[u8]) -> UTXOCacheEvent {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const UTXOCacheEvent) }
    }
}

pub const UTXOCACHE_FLUSHMODE_NONE: u32 = 0;
pub const UTXOCACHE_FLUSHMODE_IFNEEDED: u32 = 1;
pub const UTXOCACHE_FLUSHMODE_PERIODIC: u32 = 2;
pub const UTXOCACHE_FLUSHMODE_ALWAYS: u32 = 3;

/// Represents an UTXO cache flush.
#[repr(C)]
pub struct UTXOCacheFlush {
    pub duration: u64,
    pub mode: u32,
    pub coins_count: u64,
    pub coins_memusage: u64,
    pub flush_for_prune: bool,
}

impl UTXOCacheFlush {
    pub fn from_bytes(x: &[u8]) -> UTXOCacheFlush {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const UTXOCacheFlush) }
    }

    pub fn flush_mode(&self) -> &str {
        match self.mode {
            UTXOCACHE_FLUSHMODE_NONE => "NONE",
            UTXOCACHE_FLUSHMODE_IFNEEDED => "IF_NEEDED",
            UTXOCACHE_FLUSHMODE_PERIODIC => "PERIODIC",
            UTXOCACHE_FLUSHMODE_ALWAYS => "ALWAYS",
            _ => "UNKNOWN",
        }
    }

    pub fn flush_for_prune(&self) -> &str {
        if self.flush_for_prune {
            "true"
        } else {
            "false"
        }
    }
}

/// Represents an added mempool transaction.
#[repr(C)]
pub struct MempoolAdded {
    pub vsize: u64,
    pub fee: i64,
}

impl MempoolAdded {
    pub fn from_bytes(x: &[u8]) -> MempoolAdded {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const MempoolAdded) }
    }
}

const MAX_REMOVAL_REASON_LENGTH: usize = 9;

/// Represents a removed mempool transaction.
#[repr(C)]
pub struct MempoolRemoved {
    pub reason: [u8; MAX_REMOVAL_REASON_LENGTH],
    pub vsize: u64,
    pub fee: i64,
}

impl MempoolRemoved {
    pub fn from_bytes(x: &[u8]) -> MempoolRemoved {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const MempoolRemoved) }
    }

    pub fn removal_reason(&self) -> String {
        String::from_utf8_lossy(&self.reason.split(|c| *c == 0x00u8).next().unwrap()).into_owned()
    }
}

const MAX_REJECT_REASON_LENGTH: usize = 118;

/// Represents a rejected mempool transaction.
#[repr(C)]
pub struct MempoolRejected {
    pub reason: [u8; MAX_REJECT_REASON_LENGTH],
}

impl MempoolRejected {
    pub fn from_bytes(x: &[u8]) -> MempoolRejected {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const MempoolRejected) }
    }

    pub fn reject_reason(&self) -> String {
        String::from_utf8_lossy(&self.reason.split(|c| *c == 0x00u8).next().unwrap()).into_owned()
    }
}
