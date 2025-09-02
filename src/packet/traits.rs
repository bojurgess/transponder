use bytemuck::Pod;

use crate::{packet::PacketError, raw::header::PacketHeader};

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
