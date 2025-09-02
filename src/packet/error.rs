use core::fmt;

#[derive(Debug)]
pub enum PacketError {
    InvalidLength { expected: usize, actual: usize },
    InvalidData,
    InvalidHeader(String),
    BytemuckError(String),
    Unknown(String),
}

impl std::error::Error for PacketError {}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketError::InvalidLength { expected, actual } => {
                write!(
                    f,
                    "Invalid packet length: expected {}, got {}",
                    expected, actual
                )
            }
            PacketError::InvalidData => write!(f, "Failed to interpret packet data"),
            PacketError::InvalidHeader(msg) => write!(f, "Invalid packet header: {}", msg),
            PacketError::BytemuckError(msg) => write!(f, "Bytemuck error: {}", msg),
            PacketError::Unknown(msg) => write!(f, "Packet error: {}", msg),
        }
    }
}
