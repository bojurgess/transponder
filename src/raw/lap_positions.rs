use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::impl_has_header,
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

impl_has_header!(PacketLapPositionsData);

assert_packet_size!(PacketLapPositionsData, packet_sizes::LAP_POSITIONS);
