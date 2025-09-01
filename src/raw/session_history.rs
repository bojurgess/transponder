use bytemuck::{Pod, Zeroable};

use crate::{
    packet::{PacketError, RawPacket},
    raw::{
        PacketHeader,
        constants::{MAX_NUM_LAPS_IN_SESSION_HISTORY, MAX_TYRE_STINTS},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct LapHistoryData {
    /// Lap time in milliseconds
    pub lap_time_in_ms: u32,
    /// Sector 1 milliseconds part
    pub sector1_time_ms_part: u16,
    /// Sector 1 whole minute part
    pub sector1_time_minutes_part: u8,
    /// Sector 2 milliseconds part
    pub sector2_time_ms_part: u16,
    /// Sector 2 whole minute part
    pub sector2_time_minutes_part: u8,
    /// Sector 3 milliseconds part
    pub sector3_time_ms_part: u16,
    /// Sector 3 whole minute part
    pub sector3_time_minutes_part: u8,
    /// 0x01 bit set-lap valid, 0x02 bit set-sector 1 valid
    /// 0x04 bit set-sector 2 valid, 0x08 bit set-sector 3 valid
    pub lap_valid_bit_flags: u8,
}

//-----------------------------------------------------------------------------
// Tyre stint history data
//-----------------------------------------------------------------------------
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TyreStintHistoryData {
    /// Lap the tyre usage ends on (255 if current tyre)
    pub end_lap: u8,
    /// Actual tyres used by this driver
    pub tyre_actual_compound: u8,
    /// Visual tyres used by this driver
    pub tyre_visual_compound: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketSessionHistoryData {
    /// Header
    pub header: PacketHeader,
    /// Index of the car this lap data relates to
    pub car_idx: u8,
    /// Num laps in the data (including current partial lap)
    pub num_laps: u8,
    /// Number of tyre stints in the data
    pub num_tyre_stints: u8,
    /// Lap the best lap time was achieved on
    pub best_lap_time_lap_num: u8,
    /// Lap the best Sector 1 time was achieved on
    pub best_sector1_lap_num: u8,
    /// Lap the best Sector 2 time was achieved on
    pub best_sector2_lap_num: u8,
    /// Lap the best Sector 3 time was achieved on
    pub best_sector3_lap_num: u8,
    /// 100 laps of data max
    pub lap_history_data: [LapHistoryData; MAX_NUM_LAPS_IN_SESSION_HISTORY],
    /// Tyre stint history data
    pub tyre_stints_history_data: [TyreStintHistoryData; MAX_TYRE_STINTS],
}

impl RawPacket for PacketSessionHistoryData {
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
