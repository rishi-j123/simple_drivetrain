mod subsystems;
mod constants;

use frcrs::input::Joystick;

use crate::constants::joystick_map::DRIVER_JOYSTICK_ID;
use crate::subsystems::drivetrain;

pub struct Ferris {
    drivetrain: drivetrain::Drivetrain,
}

impl Ferris {
    pub fn new() -> Self {
        Self{
            drivetrain: drivetrain::Drivetrain::new(),
        }
    }
}

pub struct InputSystem {
    joystick: Joystick,
}

impl InputSystem {
    pub fn new() -> Self {
        Self{
            joystick: Joystick::new(DRIVER_JOYSTICK_ID),
        }
    }
}

pub fn teleop(ferris: &Ferris, input: &InputSystem) {
    ferris.drivetrain.fr_forward(input.joystick.get_y());
}