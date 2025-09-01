use bytemuck::{Pod, Zeroable};

use crate::{
    packet::{PacketError, RawPacket},
    raw::{PacketHeader, constants::MAX_NUM_CARS},
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CarMotionData {
    pub world_position_x: f32,     // World space X position - metres
    pub world_position_y: f32,     // World space Y position
    pub world_position_z: f32,     // World space Z position
    pub world_velocity_x: f32,     // Velocity in world space X - metres/s
    pub world_velocity_y: f32,     // Velocity in world space Y
    pub world_velocity_z: f32,     // Velocity in world space Z
    pub world_forward_dir_x: i16,  // World space forward X direction (normalised)
    pub world_forward_dir_y: i16,  // World space forward Y direction (normalised)
    pub world_forward_dir_z: i16,  // World space forward Z direction (normalised)
    pub world_right_dir_x: i16,    // World space right X direction (normalised)
    pub world_right_dir_y: i16,    // World space right Y direction (normalised)
    pub world_right_dir_z: i16,    // World space right Z direction (normalised)
    pub g_force_lateral: f32,      // Lateral G-Force component
    pub g_force_longitudinal: f32, // Longitudinal G-Force component
    pub g_force_vertical: f32,     // Vertical G-Force component
    pub yaw: f32,                  // Yaw angle in radians
    pub pitch: f32,                // Pitch angle in radians
    pub roll: f32,                 // Roll angle in radians
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketMotionData {
    header: PacketHeader, // Header
    car_motion_data: [CarMotionData; MAX_NUM_CARS],
}

impl RawPacket for PacketMotionData {
    fn header(&self) -> &PacketHeader {
        &self.header
    }
    fn from_bytes(bytes: &[u8]) -> Result<Self, crate::packet::PacketError> {
        bytemuck::try_from_bytes(bytes)
            .map(|p| *p)
            .map_err(|_| PacketError)
    }
}
