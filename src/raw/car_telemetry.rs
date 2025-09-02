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
pub struct CarTelemetryData {
    /// Speed of car in kilometres per hour
    pub speed: u16,
    /// Amount of throttle applied (0.0 to 1.0)
    pub throttle: f32,
    /// Steering (-1.0 (full lock left) to 1.0 (full lock right))
    pub steer: f32,
    /// Amount of brake applied (0.0 to 1.0)
    pub brake: f32,
    /// Amount of clutch applied (0 to 100)
    pub clutch: u8,
    /// Gear selected (1-8, N=0, R=-1)
    pub gear: i8,
    /// Engine RPM
    pub engine_rpm: u16,
    /// 0 = off, 1 = on
    pub drs: u8,
    /// Rev lights indicator (percentage)
    pub rev_lights_percent: u8,
    /// Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
    pub rev_lights_bit_value: u16,
    /// Brakes temperature (celsius) for each wheel
    pub brakes_temperature: [u16; 4],
    /// Tyres surface temperature (celsius) for each wheel
    pub tyres_surface_temperature: [u8; 4],
    /// Tyres inner temperature (celsius) for each wheel
    pub tyres_inner_temperature: [u8; 4],
    /// Engine temperature (celsius)
    pub engine_temperature: u16,
    /// Tyre pressure (PSI) for each wheel
    pub tyres_pressure: [f32; 4],
    /// Driving surface, see appendices
    pub surface_type: [u8; 4],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketCarTelemetryData {
    /// Header
    pub header: PacketHeader,
    /// Telemetry data for all cars on track
    pub car_telemetry_data: [CarTelemetryData; MAX_NUM_CARS],
    /// Index of MFD panel open - 255 = MFD closed
    /// Single player, race – 0 = Car setup, 1 = Pits
    /// 2 = Damage, 3 = Engine, 4 = Temperatures
    /// May vary depending on game mode
    pub mfd_panel_index: u8,
    /// Secondary player MFD panel index (see above)
    pub mfd_panel_index_secondary_player: u8,
    /// Suggested gear for the player (1-8), 0 if no gear suggested
    pub suggested_gear: i8,
}

impl_has_header!(PacketCarTelemetryData);

assert_packet_size!(PacketCarTelemetryData, packet_sizes::CAR_TELEMETRY);
