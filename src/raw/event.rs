use std::fmt;

use bytemuck::{Pod, Zeroable};

use crate::{
    assert_packet_size,
    packet::{PacketError, RawPacket, impl_has_header},
    raw::{
        PacketHeader,
        constants::{event::EVENT_STRING_CODE_LEN, packet_sizes},
    },
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub union EventDataDetails {
    pub fastest_lap: FastestLap,
    pub retirement: Retirement,
    pub drs_disabled: DRSDisabled,
    pub team_mate_in_pits: TeamMateInPits,
    pub race_winner: RaceWinner,
    pub penalty: Penalty,
    pub speed_trap: SpeedTrap,
    pub start_lights: StartLights,
    pub drive_through_penalty_served: DriveThroughPenaltyServed,
    pub stop_go_penalty_served: StopGoPenaltyServed,
    pub flashback: Flashback,
    pub buttons: Buttons,
    pub overtake: Overtake,
    pub safety_car: SafetyCar,
    pub collision: Collision,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct FastestLap {
    /// Vehicle index of car achieving fastest lap
    pub vehicle_idx: u8,
    /// Lap time is in seconds
    pub lap_time: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Retirement {
    /// Vehicle index of car retiring
    pub vehicle_idx: u8,
    /// Result reason - 0 = invalid, 1 = retired, 2 = finished, 3 = terminal damage, 4 = inactive, 5 = not enough laps completed, 6 = black flagged, 7 = red flagged, 8 = mechanical failure, 9 = session skipped, 10 = session simulated
    pub reason: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DRSDisabled {
    /// 0 = Wet track, 1 = Safety car deployed, 2 = Red flag, 3 = Min lap not reached
    pub reason: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TeamMateInPits {
    /// Vehicle index of team mate
    pub vehicle_idx: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct RaceWinner {
    /// Vehicle index of the race winner
    pub vehicle_idx: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Penalty {
    /// Penalty type – see Appendices
    pub penalty_type: u8,
    /// Infringement type – see Appendices
    pub infringement_type: u8,
    /// Vehicle index of the car the penalty is applied to
    pub vehicle_idx: u8,
    /// Vehicle index of the other car involved
    pub other_vehicle_idx: u8,
    /// Time gained, or time spent doing action in seconds
    pub time: u8,
    /// Lap the penalty occurred on
    pub lap_num: u8,
    /// Number of places gained by this
    pub places_gained: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SpeedTrap {
    /// Vehicle index of the vehicle triggering speed trap
    pub vehicle_idx: u8,
    /// Top speed achieved in kilometres per hour
    pub speed: f32,
    /// Overall fastest speed in session = 1, otherwise 0
    pub is_overall_fastest_in_session: u8,
    /// Fastest speed for driver in session = 1, otherwise 0
    pub is_driver_fastest_in_session: u8,
    /// Vehicle index of the vehicle that is the fastest in this session
    pub fastest_vehicle_idx_in_session: u8,
    /// Speed of the vehicle that is the fastest in this session
    pub fastest_speed_in_session: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct StartLights {
    /// Number of lights showing
    pub num_lights: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DriveThroughPenaltyServed {
    /// Vehicle index of the vehicle serving drive through
    pub vehicle_idx: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct StopGoPenaltyServed {
    /// Vehicle index of the vehicle serving stop go
    pub vehicle_idx: u8,
    /// Time spent serving stop go in seconds
    pub stop_time: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Flashback {
    /// Frame identifier flashed back to
    pub flashback_frame_identifier: u32,
    /// Session time flashed back to
    pub flashback_session_time: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Buttons {
    /// Bit flags specifying which buttons are being pressed currently - see appendices
    pub button_status: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Overtake {
    /// Vehicle index of the vehicle overtaking
    pub overtaking_vehicle_idx: u8,
    /// Vehicle index of the vehicle being overtaken
    pub being_overtaken_vehicle_idx: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SafetyCar {
    /// 0 = No Safety Car, 1 = Full Safety Car, 2 = Virtual Safety Car, 3 = Formation Lap Safety Car
    pub safety_car_type: u8,
    /// 0 = Deployed, 1 = Returning, 2 = Returned, 3 = Resume Race
    pub event_type: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Collision {
    /// Vehicle index of the first vehicle involved in the collision
    pub vehicle1_idx: u8,
    /// Vehicle index of the second vehicle involved in the collision
    pub vehicle2_idx: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketEventData {
    pub header: PacketHeader,
    pub event_string_code: [u8; EVENT_STRING_CODE_LEN],
    pub event_details: EventDataDetails,
}

impl_has_header!(PacketEventData);

impl RawPacket for PacketEventData {
    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        let expected_len = std::mem::size_of::<PacketEventData>();
        if bytes.len() != expected_len {
            return Err(PacketError::InvalidLength {
                expected: expected_len,
                actual: bytes.len(),
            });
        }

        // --- Header ---
        let header_size = std::mem::size_of::<PacketHeader>();
        let header = *bytemuck::try_from_bytes(&bytes[0..header_size])
            .map_err(|_| PacketError::InvalidData)?;

        // --- Event String Code ---
        let mut event_string_code = [0u8; EVENT_STRING_CODE_LEN];
        event_string_code.copy_from_slice(&bytes[header_size..header_size + EVENT_STRING_CODE_LEN]);

        // --- Event Details (union) ---
        let union_start = header_size + EVENT_STRING_CODE_LEN;
        let union_len = std::mem::size_of::<EventDataDetails>();

        if bytes.len() < union_start + union_len {
            return Err(PacketError::InvalidLength {
                expected: union_start + union_len,
                actual: bytes.len(),
            });
        }

        let union_bytes = &bytes[union_start..union_start + union_len];

        let mut event_details = EventDataDetails {
            fastest_lap: FastestLap {
                vehicle_idx: 0,
                lap_time: 0.0,
            },
        };

        unsafe {
            std::ptr::copy_nonoverlapping(
                union_bytes.as_ptr(),
                &mut event_details as *mut _ as *mut u8,
                union_len,
            );
        }

        Ok(PacketEventData {
            header,
            event_string_code,
            event_details,
        })
    }

    fn into_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl fmt::Debug for PacketEventData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("PacketEventData");

        ds.field("header", &self.header);

        let code_str = std::str::from_utf8(&self.event_string_code).unwrap_or("<invalid utf8>");
        ds.field("event_string_code", &code_str);

        unsafe {
            let bytes = std::slice::from_raw_parts(
                &self.event_details as *const _ as *const u8,
                std::mem::size_of::<EventDataDetails>(),
            );
            ds.field("event_details_bytes", &bytes);
        }

        ds.finish()
    }
}

assert_packet_size!(PacketEventData, packet_sizes::EVENT);
