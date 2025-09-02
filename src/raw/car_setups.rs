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
pub struct CarSetupData {
    /// Front wing aero
    pub front_wing: u8,
    /// Rear wing aero
    pub rear_wing: u8,
    /// Differential adjustment on throttle (percentage)
    pub on_throttle: u8,
    /// Differential adjustment off throttle (percentage)
    pub off_throttle: u8,
    /// Front camber angle (suspension geometry)
    pub front_camber: f32,
    /// Rear camber angle (suspension geometry)
    pub rear_camber: f32,
    /// Front toe angle (suspension geometry)
    pub front_toe: f32,
    /// Rear toe angle (suspension geometry)
    pub rear_toe: f32,
    /// Front suspension
    pub front_suspension: u8,
    /// Rear suspension
    pub rear_suspension: u8,
    /// Front anti-roll bar
    pub front_anti_roll_bar: u8,
    /// Rear anti-roll bar
    pub rear_anti_roll_bar: u8,
    /// Front ride height
    pub front_suspension_height: u8,
    /// Rear ride height
    pub rear_suspension_height: u8,
    /// Brake pressure (percentage)
    pub brake_pressure: u8,
    /// Brake bias (percentage)
    pub brake_bias: u8,
    /// Engine braking (percentage)
    pub engine_braking: u8,
    /// Rear left tyre pressure (PSI)
    pub rear_left_tyre_pressure: f32,
    /// Rear right tyre pressure (PSI)
    pub rear_right_tyre_pressure: f32,
    /// Front left tyre pressure (PSI)
    pub front_left_tyre_pressure: f32,
    /// Front right tyre pressure (PSI)
    pub front_right_tyre_pressure: f32,
    /// Ballast
    pub ballast: u8,
    /// Fuel load
    pub fuel_load: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketCarSetupData {
    /// Header
    pub header: PacketHeader,
    /// Car setup data for all cars
    pub car_setup_data: [CarSetupData; MAX_NUM_CARS],
    /// Value of front wing after next pit stop - player only
    pub next_front_wing_value: f32,
}

impl_has_header!(PacketCarSetupData);

assert_packet_size!(PacketCarSetupData, packet_sizes::CAR_SETUPS);
