pub const MAX_NUM_CARS: usize = 22;
pub const MAX_PARTICIPANT_NAME_LEN: usize = 32;
pub const MAX_TYRE_STINTS: usize = 8;
/// 13 slick and 7 wet weather
pub const MAX_TYRE_SETS: usize = 13 + 7;

pub const MAX_MARSHALLS_ZONE_PER_LAP: usize = 21;
pub const MAX_WEATHER_FORECAST_SAMPLES: usize = 64;
pub const MAX_SESSIONS_IN_WEEKEND: usize = 12;

pub const MAX_NUM_LAPS_IN_SESSION_HISTORY: usize = 100;

pub const MAX_NUM_LAPS_IN_LAP_POSITIONS_HISTORY: usize = 50;

pub mod packet_sizes {
    // Packet sizes in bytes, as defined by the telemetry spec
    pub const HEADER: usize = 29;
    pub const MOTION: usize = 1349;
    pub const SESSION: usize = 753;
    pub const LAP: usize = 1285;
    pub const EVENT: usize = 45;
    pub const PARTICIPANTS: usize = 1284;
    pub const CAR_SETUPS: usize = 1133;
    pub const CAR_TELEMETRY: usize = 1352;
    pub const CAR_STATUS: usize = 1239;
    pub const FINAL_CLASSIFICATION: usize = 1042;
    pub const LOBBY_INFO: usize = 954;
    pub const CAR_DAMAGE: usize = 1041;
    pub const SESSION_HISTORY: usize = 1460;
    pub const TYRE_SETS: usize = 231;
    pub const MOTION_EX: usize = 273;
    pub const TIME_TRIAL: usize = 101;
    pub const LAP_POSITIONS: usize = 1131;
}

pub mod event {
    pub const EVENT_STRING_CODE_LEN: usize = 4;

    pub const SESSION_STARTED_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"SSTA";
    pub const SESSION_ENDED_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"SEND";
    pub const FASTEST_LAP_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"FTLP";
    pub const RETIREMENT_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"RTMT";
    pub const DRS_ENABLED_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"DRSE";
    pub const DRS_DISABLED_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"DRSD";
    pub const TEAM_MATE_IN_PITS_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"TMPT";
    pub const CHEQUERED_FLAG_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"CHQF";
    pub const RACE_WINNER_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"RCWN";
    pub const PENALTY_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"PENA";
    pub const SPEED_TRAP_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"SPTP";
    pub const START_LIGHTS_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"STLG";
    pub const LIGHTS_OUT_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"LGOT";
    pub const DRIVE_THROUGH_SERVED_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"DTSV";
    pub const STOP_GO_SERVED_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"SGSV";
    pub const FLASHBACK_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"FLBK";
    pub const BUTTON_STATUS_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"BUTN";
    pub const RED_FLAG_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"RDFL";
    pub const OVERTAKE_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"OVTK";
    pub const SAFETY_CAR_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"SCAR";
    pub const COLLISION_EVENT_CODE: &[u8; EVENT_STRING_CODE_LEN] = b"COLL";
}
