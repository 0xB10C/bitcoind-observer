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
