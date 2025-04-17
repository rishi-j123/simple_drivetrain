use crate::constants::drivetrain;
use frcrs::ctre::Talon;

pub struct Drivetrain{
    fr_drive: Talon,
    fr_turn: Talon,

    fl_drive: Talon,
    fl_turn: Talon,

    br_drive: Talon,
    br_turn: Talon,

    bl_drive: Talon,
    bl_turn: Talon,
}