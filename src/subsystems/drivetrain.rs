#![allow(dead_code)]

#[allow(unused)]
use frcrs::ctre::{Talon, ControlMode, CanCoder};
#[allow(unused)]
use crate::constants::robomap::drivetrain::{BL_DRIVE, BL_ENCODER, BL_TURN, BR_DRIVE, BR_ENCODER, BR_TURN, FL_DRIVE, FL_ENCODER, FL_TURN, FR_DRIVE, FR_ENCODER, FR_TURN};

pub struct Drivetrain{
    fr_drive: Talon,
    // fr_turn: Talon,
    // fr_encoder: CanCoder,
    //
    // fl_drive: Talon,
    // fl_turn: Talon,
    // fl_encoder: CanCoder,
    //
    // br_drive: Talon,
    // br_turn: Talon,
    // br_encoder: CanCoder,
    //
    // bl_drive: Talon,
    // bl_turn: Talon,
    // bl_encoder: CanCoder,
}


impl Drivetrain {
    pub fn new() -> Self {
        Self {
            fr_drive: Talon::new(FR_DRIVE, None),
            // fr_turn: Talon::new(FR_TURN, None),
            // fr_encoder: CanCoder::new(FR_ENCODER, None),
            //
            // fl_drive: Talon::new(FL_DRIVE, None),
            // fl_turn: Talon::new(FL_TURN, None),
            // fl_encoder: CanCoder::new(FL_ENCODER, None),
            //
            // br_drive: Talon::new(BR_DRIVE, None),
            // br_turn: Talon::new(BR_TURN, None),
            // br_encoder: CanCoder::new(BR_ENCODER, None),
            //
            // bl_drive: Talon::new(BL_DRIVE, None),
            // bl_turn: Talon::new(BL_TURN, None),
            // bl_encoder: CanCoder::new(BL_ENCODER, None),
        }
    }

    //would not be useful on a robot, just to test
    pub fn fr_forward(&self, power: f64) {
        self.fr_drive.set(ControlMode::Percent, power);
    }
}
