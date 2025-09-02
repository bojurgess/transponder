use crate::utils::define_appendix;

define_appendix!(PenaltyType {
    0 => DriveThrough => "Drive Through",
    1 => StopGo => "Stop Go",
    2 => GridPenalty => "Grid Penalty",
    3 => PenaltyReminder => "Penalty Reminder",
    4 => TimePenalty => "TimePenalty",
    5 => Warning,
    6 => Disqualified,
    7 => RemovedFromFormationLap => "Removed from formation lap",
    8 => ParkedTooLongTimer => "Parked too long timer",
    9 => TyreRegulations => "Tyre Regulations",
    10 => ThisLapInvalidated => "This lap invalidated",
    11 => ThisAndNextLapInvalidated => "This and next lap invalidated",
    12 => ThisLapInvalidatedWithoutReason => "This lap invalidated without reason",
    13 => ThisAndNextLapInvalidatedWithoutReason => "This and next lap invalidated without reason",
    14 => ThisAndPreviousLapInvalidated => "This and previous lap invalidated",
    15 => ThisAndPreviousLapInvalidatedWithoutReason => "This and previous lap invalidated without reason",
    16 => Retired,
    17 => BlackFlagTimer => "Black Flag Timer"
});
