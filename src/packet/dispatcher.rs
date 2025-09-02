use crate::{
    packet::{HasHeader, PacketError, RawPacket},
    raw::*,
};

pub enum AnyRawPacket {
    CarDamage(PacketCarDamageData),
    CarSetups(PacketCarSetupData),
    CarStatus(PacketCarStatusData),
    CarTelemetry(PacketCarTelemetryData),
    Event(PacketEventData),
    FinalClassification(PacketFinalClassificationData),
    Header(PacketHeader),
    LapPositions(PacketLapPositionsData),
    Lap(PacketLapData),
    LobbyInfo(PacketLobbyInfoData),
    MotionEx(PacketMotionExData),
    Motion(PacketMotionData),
    Participants(PacketParticipantsData),
    SessionHistory(PacketSessionHistoryData),
    Session(PacketSessionData),
    TimeTrial(PacketTimeTrialData),
    TyreSets(PacketTyreSetsData),
}

impl AnyRawPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        let header_len = std::mem::size_of::<PacketHeader>();
        if bytes.len() < header_len {
            return Err(PacketError::InvalidLength {
                expected: header_len,
                actual: bytes.len(),
            });
        }

        let header = PacketHeader::from_bytes(&bytes[0..header_len])?;

        match header.packet_id {
            0 => Ok(Self::Motion(PacketMotionData::from_bytes(bytes)?)),
            1 => Ok(Self::Session(PacketSessionData::from_bytes(bytes)?)),
            2 => Ok(Self::Lap(PacketLapData::from_bytes(bytes)?)),
            3 => Ok(Self::Event(PacketEventData::from_bytes(bytes)?)),
            4 => Ok(Self::Participants(PacketParticipantsData::from_bytes(
                bytes,
            )?)),
            5 => Ok(Self::CarSetups(PacketCarSetupData::from_bytes(bytes)?)),
            6 => Ok(Self::CarTelemetry(PacketCarTelemetryData::from_bytes(
                bytes,
            )?)),
            7 => Ok(Self::CarStatus(PacketCarStatusData::from_bytes(bytes)?)),
            8 => Ok(Self::FinalClassification(
                PacketFinalClassificationData::from_bytes(bytes)?,
            )),
            9 => Ok(Self::LobbyInfo(PacketLobbyInfoData::from_bytes(bytes)?)),
            10 => Ok(Self::CarDamage(PacketCarDamageData::from_bytes(bytes)?)),
            11 => Ok(Self::SessionHistory(PacketSessionHistoryData::from_bytes(
                bytes,
            )?)),
            12 => Ok(Self::TyreSets(PacketTyreSetsData::from_bytes(bytes)?)),
            13 => Ok(Self::MotionEx(PacketMotionExData::from_bytes(bytes)?)),
            14 => Ok(Self::TimeTrial(PacketTimeTrialData::from_bytes(bytes)?)),
            15 => Ok(Self::LapPositions(PacketLapPositionsData::from_bytes(
                bytes,
            )?)),
            _ => Err(PacketError::InvalidData),
        }
    }

    pub fn header(&self) -> &PacketHeader {
        match self {
            AnyRawPacket::Motion(p) => p.header(),
            AnyRawPacket::Session(p) => p.header(),
            AnyRawPacket::Lap(p) => p.header(),
            AnyRawPacket::Event(p) => p.header(),
            AnyRawPacket::Participants(p) => p.header(),
            AnyRawPacket::CarSetups(p) => p.header(),
            AnyRawPacket::CarTelemetry(p) => p.header(),
            AnyRawPacket::CarStatus(p) => p.header(),
            AnyRawPacket::FinalClassification(p) => p.header(),
            AnyRawPacket::LobbyInfo(p) => p.header(),
            AnyRawPacket::CarDamage(p) => p.header(),
            AnyRawPacket::SessionHistory(p) => p.header(),
            AnyRawPacket::TyreSets(p) => p.header(),
            AnyRawPacket::MotionEx(p) => p.header(),
            AnyRawPacket::TimeTrial(p) => p.header(),
            AnyRawPacket::LapPositions(p) => p.header(),
            AnyRawPacket::Header(p) => p,
        }
    }
}
