use bytemuck::{Pod, Zeroable};

use crate::{
    packet::{PacketError, RawPacket},
    raw::{PacketHeader, constants::MAX_NUM_CARS},
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct LapData {
    /// Last lap time in milliseconds
    pub last_lap_time_in_ms: u32,
    /// Current time around the lap in milliseconds
    pub current_lap_time_in_ms: u32,
    /// Sector 1 time milliseconds part
    pub sector1_time_ms_part: u16,
    /// Sector 1 whole minute part
    pub sector1_time_minutes_part: u8,
    /// Sector 2 time milliseconds part
    pub sector2_time_ms_part: u16,
    /// Sector 2 whole minute part
    pub sector2_time_minutes_part: u8,
    /// Time delta to car in front milliseconds part
    pub delta_to_car_in_front_ms_part: u16,
    /// Time delta to car in front whole minute part
    pub delta_to_car_in_front_minutes_part: u8,
    /// Time delta to race leader milliseconds part
    pub delta_to_race_leader_ms_part: u16,
    /// Time delta to race leader whole minute part
    pub delta_to_race_leader_minutes_part: u8,
    /// Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
    pub lap_distance: f32,
    /// Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
    pub total_distance: f32,
    /// Delta in seconds for safety car
    pub safety_car_delta: f32,
    /// Car race position
    pub car_position: u8,
    /// Current lap number
    pub current_lap_num: u8,
    /// 0 = none, 1 = pitting, 2 = in pit area
    pub pit_status: u8,
    /// Number of pit stops taken in this race
    pub num_pit_stops: u8,
    /// 0 = sector1, 1 = sector2, 2 = sector3
    pub sector: u8,
    /// Current lap invalid - 0 = valid, 1 = invalid
    pub current_lap_invalid: u8,
    /// Accumulated time penalties in seconds to be added
    pub penalties: u8,
    /// Accumulated number of warnings issued
    pub total_warnings: u8,
    /// Accumulated number of corner cutting warnings issued
    pub corner_cutting_warnings: u8,
    /// Num drive through pens left to serve
    pub num_unserved_drive_through_pens: u8,
    /// Num stop go pens left to serve
    pub num_unserved_stop_go_pens: u8,
    /// Grid position the vehicle started the race in
    pub grid_position: u8,
    /// Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
    pub driver_status: u8,
    /// Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished,
    /// 4 = did not finish, 5 = disqualified, 6 = not classified, 7 = retired
    pub result_status: u8,
    /// Pit lane timing, 0 = inactive, 1 = active
    pub pit_lane_timer_active: u8,
    /// If active, the current time spent in the pit lane in ms
    pub pit_lane_time_in_lane_in_ms: u16,
    /// Time of the actual pit stop in ms
    pub pit_stop_timer_in_ms: u16,
    /// Whether the car should serve a penalty at this stop
    pub pit_stop_should_serve_pen: u8,
    /// Fastest speed through speed trap for this car in kmph
    pub speed_trap_fastest_speed: f32,
    /// Lap no the fastest speed was achieved, 255 = not set
    pub speed_trap_fastest_lap: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PacketLapData {
    /// Header
    pub header: PacketHeader,
    // Packet specific data
    /// Lap data for all cars on track
    pub lap_data: [LapData; MAX_NUM_CARS],
    /// Index of Personal Best car in time trial (255 if invalid)
    pub time_trial_pb_car_idx: u8,
    /// Index of Rival car in time trial (255 if invalid)
    pub time_trial_rival_car_idx: u8,
}

impl RawPacket for PacketLapData {
    fn header(&self) -> &PacketHeader {
        &self.header
    }
    fn from_bytes(bytes: &[u8]) -> Result<Self, crate::packet::PacketError> {
        bytemuck::try_from_bytes(bytes)
            .map(|p| *p)
            .map_err(|_| PacketError)
    }
}
