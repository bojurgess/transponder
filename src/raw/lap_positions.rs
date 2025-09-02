use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::{PacketError, RawPacket},
    raw::{
        PacketHeader,
        constants::{MAX_NUM_CARS, MAX_NUM_LAPS_IN_LAP_POSITIONS_HISTORY, packet_sizes},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketLapPositionsData {
    /// Header
    pub header: PacketHeader,
    /// Number of laps in the data
    pub num_laps: u8,
    /// Index of the lap where the data starts, 0 indexed
    pub lap_start: u8,
    /// Array holding the position of the car in a given lap, 0 if no record
    pub position_for_vehicle_idx: [[u8; MAX_NUM_LAPS_IN_LAP_POSITIONS_HISTORY]; MAX_NUM_CARS],
}

impl RawPacket for PacketLapPositionsData {
    fn header(&self) -> &PacketHeader {
        &self.header
    }
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
}

assert_packet_size!(PacketLapPositionsData, packet_sizes::LAP_POSITIONS);
