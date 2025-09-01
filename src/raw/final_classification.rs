use bytemuck::{Pod, Zeroable};

use crate::{
    packet::{PacketError, RawPacket},
    raw::{
        PacketHeader,
        constants::{MAX_NUM_CARS, MAX_TYRE_STINTS},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct FinalClassificationData {
    /// Finishing position
    pub position: u8,
    /// Number of laps completed
    pub num_laps: u8,
    /// Grid position of the car
    pub grid_position: u8,
    /// Number of points scored
    pub points: u8,
    /// Number of pit stops made
    pub num_pit_stops: u8,
    /// Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = didnotfinish, 5 = disqualified, 6 = not classified, 7 = retired
    pub result_status: u8,
    /// Result reason - 0 = invalid, 1 = retired, 2 = finished, 3 = terminal damage, 4 = inactive, 5 = not enough laps completed, 6 = black flagged
    /// 7 = red flagged, 8 = mechanical failure, 9 = session skipped, 10 = session simulated
    pub result_reason: u8,
    /// Best lap time of the session in milliseconds
    pub best_lap_time_in_ms: u32,
    /// Total race time in seconds without penalties
    pub total_race_time: f64,
    /// Total penalties accumulated in seconds
    pub penalties_time: u8,
    /// Number of penalties applied to this driver
    pub num_penalties: u8,
    /// Number of tyres stints up to maximum
    pub num_tyre_stints: u8,
    /// Actual tyres used by this driver
    pub tyre_stints_actual: [u8; MAX_TYRE_STINTS],
    /// Visual tyres used by this driver
    pub tyre_stints_visual: [u8; MAX_TYRE_STINTS],
    /// The lap number stints end on
    pub tyre_stints_end_laps: [u8; MAX_TYRE_STINTS],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketFinalClassificationData {
    /// Header
    pub header: PacketHeader,
    /// Number of cars in the final classification
    pub num_cars: u8,
    /// Final classification data for all cars
    pub classification_data: [FinalClassificationData; MAX_NUM_CARS],
}

impl RawPacket for PacketFinalClassificationData {
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
