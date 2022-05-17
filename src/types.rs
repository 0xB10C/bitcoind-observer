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

pub const MAX_LOCK_NAME: usize = 16;
pub const MAX_FILE_NAME: usize = 32;

/// Represents a sync (lock) event.
#[repr(C)]
pub struct SyncEvent {
    pub mutex: u64,
    pub lock_name: [u8; MAX_LOCK_NAME],
    pub file_name: [u8; MAX_FILE_NAME],
    pub line_number: u64,
}

impl SyncEvent {
    pub fn from_bytes(x: &[u8]) -> SyncEvent {
        unsafe { ptr::read_unaligned(x.as_ptr() as *const SyncEvent) }
    }

    pub fn lock_name(&self) -> String {
        String::from_utf8_lossy(&self.lock_name.split(|c| *c == 0x00u8).next().unwrap())
            .into_owned()
    }

    pub fn file_name(&self) -> String {
        String::from_utf8_lossy(&self.file_name.split(|c| *c == 0x00u8).next().unwrap())
            .into_owned()
    }
}

impl fmt::Display for SyncEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SyncEvent(mutex={}, {} in {}:{})",
            self.mutex,
            self.lock_name(),
            self.file_name(),
            self.line_number,
        )
    }
}
