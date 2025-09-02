use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::{PacketError, RawPacket},
    raw::{
        PacketHeader,
        constants::{MAX_NUM_CARS, packet_sizes},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CarDamageData {
    /// Tyre wear (percentage)
    pub tyres_wear: [f32; 4],
    /// Tyre damage (percentage)
    pub tyres_damage: [u8; 4],
    /// Brakes damage (percentage)
    pub brakes_damage: [u8; 4],
    /// Tyre blisters value (percentage)
    pub tyre_blisters: [u8; 4],
    /// Front left wing damage (percentage)
    pub front_left_wing_damage: u8,
    /// Front right wing damage (percentage)
    pub front_right_wing_damage: u8,
    /// Rear wing damage (percentage)
    pub rear_wing_damage: u8,
    /// Floor damage (percentage)
    pub floor_damage: u8,
    /// Diffuser damage (percentage)
    pub diffuser_damage: u8,
    /// Sidepod damage (percentage)
    pub sidepod_damage: u8,
    /// Indicator for DRS fault, 0 = OK, 1 = fault
    pub drs_fault: u8,
    /// Indicator for ERS fault, 0 = OK, 1 = fault
    pub ers_fault: u8,
    /// Gear box damage (percentage)
    pub gear_box_damage: u8,
    /// Engine damage (percentage)
    pub engine_damage: u8,
    /// Engine wear MGU-H (percentage)
    pub engine_mguh_wear: u8,
    /// Engine wear ES (percentage)
    pub engine_es_wear: u8,
    /// Engine wear CE (percentage)
    pub engine_ce_wear: u8,
    /// Engine wear ICE (percentage)
    pub engine_ice_wear: u8,
    /// Engine wear MGU-K (percentage)
    pub engine_mguk_wear: u8,
    /// Engine wear TC (percentage)
    pub engine_tc_wear: u8,
    /// Engine blown, 0 = OK, 1 = fault
    pub engine_blown: u8,
    /// Engine seized, 0 = OK, 1 = fault
    pub engine_seized: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketCarDamageData {
    /// Header
    pub header: PacketHeader,
    /// Car damage data for all cars on track
    pub car_damage_data: [CarDamageData; MAX_NUM_CARS],
}

impl RawPacket for PacketCarDamageData {
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

assert_packet_size!(PacketCarDamageData, packet_sizes::CAR_DAMAGE);
