use core::fmt;

use crate::raw::header::PacketHeader;

pub trait RawPacket: Sized {
    fn header(&self) -> &PacketHeader;
    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError>;
}

pub trait Packet: Sized {
    type Raw: RawPacket;

    fn from_raw(raw: Self::Raw) -> Result<Self, PacketError>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        let raw = Self::Raw::from_bytes(bytes)?;
        Self::from_raw(raw)
    }
}

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
