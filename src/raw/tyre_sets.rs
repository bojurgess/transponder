use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::impl_has_header,
    raw::{
        PacketHeader,
        constants::{MAX_TYRE_SETS, packet_sizes},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TyreSetData {
    /// Actual tyre compound used
    pub actual_tyre_compound: u8,
    /// Visual tyre compound used
    pub visual_tyre_compound: u8,
    /// Tyre wear (percentage)
    pub wear: u8,
    /// Whether this set is currently available
    pub available: u8,
    /// Recommended session for tyre set, see appendix
    pub recommended_session: u8,
    /// Laps left in this tyre set
    pub life_span: u8,
    /// Max number of laps recommended for this compound
    pub usable_life: u8,
    /// Lap delta time in milliseconds compared to fitted set
    pub lap_delta_time: i16,
    /// Whether the set is fitted or not
    pub fitted: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketTyreSetsData {
    /// Header
    pub header: PacketHeader,
    /// Index of the car this data relates to
    pub car_idx: u8,
    /// Tyre set data - 13 (dry) + 7 (wet)
    pub tyre_set_data: [TyreSetData; MAX_TYRE_SETS],
    /// Index into array of fitted tyre
    pub fitted_idx: u8,
}

impl_has_header!(PacketTyreSetsData);

assert_packet_size!(PacketTyreSetsData, packet_sizes::TYRE_SETS);
