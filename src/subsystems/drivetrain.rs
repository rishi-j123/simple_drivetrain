#![allow(dead_code)]

use frcrs::ctre::{Talon, ControlMode};
use crate::constants::robomap::drivetrain::{BL_DRIVE, BL_TURN, BR_DRIVE, BR_TURN, FL_DRIVE, FL_TURN, FR_DRIVE, FR_TURN};

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


impl Drivetrain {
    pub fn new() -> Self {
        Self{
            fr_drive: Talon::new(FR_DRIVE, None),
            fr_turn: Talon::new(FR_TURN, None),

            fl_drive: Talon::new(FL_DRIVE, None),
            fl_turn: Talon::new(FL_TURN, None),

            br_drive: Talon::new(BR_DRIVE, None),
            br_turn: Talon::new(BR_TURN, None),

            bl_drive: Talon::new(BL_DRIVE, None),
            bl_turn: Talon::new(BL_TURN, None),
        }
    }

    //would not be useful on a robot, just to test
    pub fn power_all(&self, power: f64) {
        self.fr_turn.set(ControlMode::Percent, power);
    }
}