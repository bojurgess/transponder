use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::impl_has_header,
    raw::{PacketHeader, constants::packet_sizes},
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketMotionExData {
    /// Header
    pub header: PacketHeader,
    /// Suspension position for each wheel: RL, RR, FL, FR
    pub suspension_position: [f32; 4],
    /// Suspension velocity for each wheel: RL, RR, FL, FR
    pub suspension_velocity: [f32; 4],
    /// Suspension acceleration for each wheel: RL, RR, FL, FR
    pub suspension_acceleration: [f32; 4],
    /// Speed of each wheel
    pub wheel_speed: [f32; 4],
    /// Slip ratio for each wheel
    pub wheel_slip_ratio: [f32; 4],
    /// Slip angles for each wheel
    pub wheel_slip_angle: [f32; 4],
    /// Lateral forces for each wheel
    pub wheel_lat_force: [f32; 4],
    /// Longitudinal forces for each wheel
    pub wheel_long_force: [f32; 4],
    /// Height of centre of gravity above ground
    pub height_of_cog_above_ground: f32,
    /// Velocity in local space X - metres/s
    pub local_velocity_x: f32,
    /// Velocity in local space Y - metres/s
    pub local_velocity_y: f32,
    /// Velocity in local space Z - metres/s
    pub local_velocity_z: f32,
    /// Angular velocity x-component - radians/s
    pub angular_velocity_x: f32,
    /// Angular velocity y-component - radians/s
    pub angular_velocity_y: f32,
    /// Angular velocity z-component - radians/s
    pub angular_velocity_z: f32,
    /// Angular acceleration x-component - radians/s²
    pub angular_acceleration_x: f32,
    /// Angular acceleration y-component - radians/s²
    pub angular_acceleration_y: f32,
    /// Angular acceleration z-component - radians/s²
    pub angular_acceleration_z: f32,
    /// Current front wheels angle in radians
    pub front_wheels_angle: f32,
    /// Vertical forces for each wheel
    pub wheel_vert_force: [f32; 4],
    /// Front plank edge height above road surface
    pub front_aero_height: f32,
    /// Rear plank edge height above road surface
    pub rear_aero_height: f32,
    /// Roll angle of the front suspension
    pub front_roll_angle: f32,
    /// Roll angle of the rear suspension
    pub rear_roll_angle: f32,
    /// Yaw angle of the chassis relative to the direction of motion - radians
    pub chassis_yaw: f32,
    /// Pitch angle of the chassis relative to the direction of motion - radians
    pub chassis_pitch: f32,
    /// Camber of each wheel in radians
    pub wheel_camber: [f32; 4],
    /// Camber gain for each wheel in radians, difference between active camber and dynamic camber
    pub wheel_camber_gain: [f32; 4],
}

impl_has_header!(PacketMotionExData);

assert_packet_size!(PacketMotionExData, packet_sizes::MOTION_EX);
