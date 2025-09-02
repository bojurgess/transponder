use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::impl_has_header,
    raw::{
        PacketHeader,
        constants::{MAX_NUM_CARS, packet_sizes},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CarStatusData {
    /// Traction control - 0 = off, 1 = medium, 2 = full
    pub traction_control: u8,
    /// 0 (off) - 1 (on)
    pub anti_lock_brakes: u8,
    /// Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub fuel_mix: u8,
    /// Front brake bias (percentage)
    pub front_brake_bias: u8,
    /// Pit limiter status - 0 = off, 1 = on
    pub pit_limiter_status: u8,
    /// Current fuel mass
    pub fuel_in_tank: f32,
    /// Fuel capacity
    pub fuel_capacity: f32,
    /// Fuel remaining in terms of laps (value on MFD)
    pub fuel_remaining_laps: f32,
    /// Cars max RPM, point of rev limiter
    pub max_rpm: u16,
    /// Cars idle RPM
    pub idle_rpm: u16,
    /// Maximum number of gears
    pub max_gears: u8,
    /// 0 = not allowed, 1 = allowed
    pub drs_allowed: u8,
    /// 0 = DRS not available, non-zero - DRS will be available in [X] metres
    pub drs_activation_distance: u16,
    /// F1 Modern / Classic / F2 tyre compound codes (see comment above)
    pub actual_tyre_compound: u8,
    /// F1 visual (can differ from actual compound) (see comment above)
    pub visual_tyre_compound: u8,
    /// Age in laps of the current set of tyres
    pub tyres_age_laps: u8,
    /// -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
    pub vehicle_fia_flags: i8,
    /// Engine power output of ICE (W)
    pub engine_power_ice: f32,
    /// Engine power output of MGU-K (W)
    pub engine_power_mguk: f32,
    /// ERS energy store in Joules
    pub ers_store_energy: f32,
    /// ERS deployment mode, 0 = none, 1 = medium, 2 = hotlap, 3 = overtake
    pub ers_deploy_mode: u8,
    /// ERS energy harvested this lap by MGU-K
    pub ers_harvested_this_lap_mguk: f32,
    /// ERS energy harvested this lap by MGU-H
    pub ers_harvested_this_lap_mguh: f32,
    /// ERS energy deployed this lap
    pub ers_deployed_this_lap: f32,
    /// Whether the car is paused in a network game
    pub network_paused: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketCarStatusData {
    /// Header
    pub header: PacketHeader,
    /// Car status data for all cars on track
    pub car_status_data: [CarStatusData; MAX_NUM_CARS],
}

impl_has_header!(PacketCarStatusData);

assert_packet_size!(PacketCarStatusData, packet_sizes::CAR_STATUS);
