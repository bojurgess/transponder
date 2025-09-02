use crate::{
    assert_packet_size,
    packet::impl_has_header,
    raw::{
        PacketHeader,
        constants::{MAX_NUM_CARS, MAX_PARTICIPANT_NAME_LEN, packet_sizes},
    },
};
use bytemuck::{Pod, Zeroable};

/// RGB value of a colour
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct LiveryColour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ParticipantData {
    /// Whether the vehicle is AI (1) or Human (0) controlled
    pub ai_controlled: u8,
    /// Driver id - see appendix, 255 if network human
    pub driver_id: u8,
    /// Network id - unique identifier for network players
    pub network_id: u8,
    /// Team id - see appendix
    pub team_id: u8,
    /// My team flag - 1 = My Team, 0 = otherwise
    pub my_team: u8,
    /// Race number of the car
    pub race_number: u8,
    /// Nationality of the driver
    pub nationality: u8,
    /// Name of participant in UTF-8 format – null terminated
    /// Will be truncated with ... (U+2026) if too long
    pub name: [u8; MAX_PARTICIPANT_NAME_LEN],
    /// The player's UDP setting, 0 = restricted, 1 = public
    pub your_telemetry: u8,
    /// The player's show online names setting, 0 = off, 1 = on
    pub show_online_names: u8,
    /// F1 World tech level
    pub tech_level: u16,
    /// 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
    pub platform: u8,
    /// Number of colours valid for this car
    pub num_colours: u8,
    /// Colours for the car
    pub livery_colours: [LiveryColour; 4],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketParticipantsData {
    /// Header
    pub header: PacketHeader,
    /// Number of active cars in the data - should match number of cars on HUD
    pub num_active_cars: u8,
    /// Participant data for all cars
    pub participants: [ParticipantData; MAX_NUM_CARS],
}

impl_has_header!(PacketParticipantsData);

assert_packet_size!(PacketParticipantsData, packet_sizes::PARTICIPANTS);
