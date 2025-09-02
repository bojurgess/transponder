use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::{PacketError, RawPacket},
    raw::constants::packet_sizes,
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketHeader {
    pub packet_format: u16,             // 2025
    pub game_year: u8,                  // Game year - last two digits e.g. 25
    pub game_major_version: u8,         // Game major version - "X.00"
    pub game_minor_version: u8,         // Game minor version - "1.XX"
    pub packet_version: u8,             // Version of this packet type
    pub packet_id: u8,                  // Identifier for the packet type
    pub session_uid: u64,               // Unique identifier for the session
    pub session_time: f32,              // Session timestamp
    pub frame_identifier: u32,          // Identifier for the frame the data was retrieved on
    pub overall_frame_identifier: u32, // Overall identifier for the frame the data was retrieved on, doesn't go back after flashbacks
    pub player_car_index: u8,          // Index of player's car in the array
    pub secondary_player_car_index: u8, // Index of secondary player's car in the array (splitscreen) - 255 if no second player
}

impl RawPacket for PacketHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        let expected_len = std::mem::size_of::<Self>();
        if bytes.len() != expected_len {
            return Err(PacketError::InvalidLength {
                expected: expected_len,
                actual: bytes.len(),
            });
        }
        bytemuck::try_from_bytes(bytes)
            .map(|p| *p)
            .map_err(|_| PacketError::InvalidData)
    }

    fn into_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

assert_packet_size!(PacketHeader, packet_sizes::HEADER);
