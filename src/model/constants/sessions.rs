use crate::utils::define_appendix;

define_appendix!(SessionType {
    0 => Unknown,
    1 => Practice1 => "Practice 1",
    2 => Practice2 => "Practice 2",
    3 => Practice3 => "Practice 3",
    4 => ShortPractice => "Short Practice",
    5 => Qualifying1 => "Qualifying 1",
    6 => Qualifying2 => "Qualifying 2",
    7 => Qualifying3 => "Qualifying 3",
    8 => ShortQualifying => "Short Qualifying",
    9 => OneShotQualifying => "One Shot Qualifying",
    10 => SprintShootout1 => "Sprint Shootout 1",
    11 => SprintShootout2 => "Sprint Shooutout 2",
    12 => SprintShootout3 => "Sprint Shootout 3",
    13 => ShortSprintShootout => "Short Sprint Shootout",
    14 => OneShotSprintShootout => "One Shot Sprint Shootout",
    15 => Race,
    16 => Race2 => "Race 2",
    17 => Race3 => "Race 3",
    18 => TimeTrial => "Time Trial"
});
