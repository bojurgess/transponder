use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::{PacketError, RawPacket},
    raw::{
        PacketHeader,
        constants::{MAX_NUM_CARS, MAX_PARTICIPANT_NAME_LEN, packet_sizes},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct LobbyInfoData {
    /// Whether the vehicle is AI (1) or Human (0) controlled
    pub ai_controlled: u8,
    /// Team id - see appendix (255 if no team currently selected)
    pub team_id: u8,
    /// Nationality of the driver
    pub nationality: u8,
    /// Platform: 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
    pub platform: u8,
    /// Name of participant in UTF-8 format – null terminated
    /// Will be truncated with ... (U+2026) if too long
    pub name: [u8; MAX_PARTICIPANT_NAME_LEN],
    /// Car number of the player
    pub car_number: u8,
    /// The player's UDP setting, 0 = restricted, 1 = public
    pub your_telemetry: u8,
    /// The player's show online names setting, 0 = off, 1 = on
    pub show_online_names: u8,
    /// F1 World tech level
    pub tech_level: u16,
    /// 0 = not ready, 1 = ready, 2 = spectating
    pub ready_status: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketLobbyInfoData {
    /// Header
    pub header: PacketHeader,
    /// Number of players in the lobby data
    pub num_players: u8,
    /// Lobby info data for all players
    pub lobby_players: [LobbyInfoData; MAX_NUM_CARS],
}

impl RawPacket for PacketLobbyInfoData {
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

assert_packet_size!(PacketLobbyInfoData, packet_sizes::LOBBY_INFO);
