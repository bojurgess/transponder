use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::impl_has_header,
    raw::{PacketHeader, constants::packet_sizes},
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TimeTrialDataSet {
    /// Index of the car this data relates to
    pub car_idx: u8,
    /// Team id - see appendix
    pub team_id: u8,
    /// Lap time in milliseconds
    pub lap_time_in_ms: u32,
    /// Sector 1 time in milliseconds
    pub sector1_time_in_ms: u32,
    /// Sector 2 time in milliseconds
    pub sector2_time_in_ms: u32,
    /// Sector 3 time in milliseconds
    pub sector3_time_in_ms: u32,
    /// 0 = assist off, 1 = assist on
    pub traction_control: u8,
    /// 0 = assist off, 1 = assist on
    pub gearbox_assist: u8,
    /// 0 = assist off, 1 = assist on
    pub anti_lock_brakes: u8,
    /// 0 = Realistic, 1 = Equal
    pub equal_car_performance: u8,
    /// 0 = No, 1 = Yes
    pub custom_setup: u8,
    /// 0 = invalid, 1 = valid
    pub valid: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketTimeTrialData {
    /// Header
    pub header: PacketHeader,
    /// Player session best data set
    pub player_session_best_data_set: TimeTrialDataSet,
    /// Personal best data set
    pub personal_best_data_set: TimeTrialDataSet,
    /// Rival data set
    pub rival_data_set: TimeTrialDataSet,
}

impl_has_header!(PacketTimeTrialData);

assert_packet_size!(PacketTimeTrialData, packet_sizes::TIME_TRIAL);
