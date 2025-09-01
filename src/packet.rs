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
pub struct PacketError;
