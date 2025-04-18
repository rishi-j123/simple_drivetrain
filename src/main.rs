#![allow(unused)]
use std::time::{Duration, Instant};
use tokio::{runtime::Runtime, time::sleep, task::LocalSet};

use frcrs::input::{Joystick, RobotState};
use frcrs::{hal_report, init_hal, observe_user_program_starting, refresh_data};

use simple_drivetrain::*;

fn main(){
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();

    executor.block_on(async{
        if !init_hal(){
            panic!("Failed to initialize HAL");
        }

        observe_user_program_starting();

        let mut last_loop_timer = Instant::now();
        let mut ferris = Ferris::new();
        let mut inputs = InputSystem::new();

        loop {
            refresh_data();
            let state = RobotState::get();
            let elapsed =  last_loop_timer.elapsed().as_secs_f64();
            let left =(1.0 / 500.0 - elapsed).max(0.0);
            if state.enabled() && state.teleop() {
                teleop(&ferris, &inputs);
            }
        }
    });
}
