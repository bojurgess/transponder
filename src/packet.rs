use core::fmt;

use bytemuck::Pod;

use crate::raw::header::PacketHeader;

pub trait HasHeader {
    fn header(&self) -> &PacketHeader;
}

pub trait RawPacket: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError>;
    fn into_bytes(&self) -> &[u8];
}

pub trait Packet: Sized {
    type Raw: RawPacket;

    fn from_raw(raw: Self::Raw) -> Result<Self, PacketError>;
    fn into_raw(self) -> Self::Raw;

    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        let raw = Self::Raw::from_bytes(bytes)?;
        Self::from_raw(raw)
    }
    fn into_bytes(&self) -> &[u8];
}

impl<T> RawPacket for T
where
    T: Pod + Sized + HasHeader,
{
    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        let expected_len = std::mem::size_of::<Self>();
        if bytes.len() != expected_len {
            return Err(PacketError::InvalidLength {
                expected: expected_len,
                actual: bytes.len(),
            });
        }

        bytemuck::try_from_bytes::<Self>(bytes)
            .map(|p| *p)
            .map_err(|e| PacketError::BytemuckError(e.to_string()))
    }
    fn into_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
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

#[macro_export]
macro_rules! assert_packet_size {
    ($struct:ty, $size:expr) => {
        const _: () = {
            use std::mem::size_of;
            assert!(
                size_of::<$struct>() == $size,
                concat!("Size mismatch for ", stringify!($struct))
            );
        };
    };
}

macro_rules! impl_has_header {
    ($ty:ty) => {
        use crate::packet::HasHeader;
        impl HasHeader for $ty {
            fn header(&self) -> &PacketHeader {
                &self.header
            }
        }
    };
}

pub(crate) use impl_has_header;
