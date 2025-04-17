pub mod robomap{
    pub mod drivetrain{
        pub const FR_DRIVE: i32 = 1;
        pub const FR_TURN: i32 = 2;

        pub const FL_DRIVE: i32 = 3;
        pub const FL_TURN: i32 = 4;

        pub const BR_DRIVE: i32 = 5;
        pub const BR_TURN: i32 = 6;

        pub const BL_DRIVE: i32 = 7;
        pub const BL_TURN: i32 = 8;
    }
}

pub mod joystick_map{
    pub const DRIVER_JOYSTICK_ID: i32 = 1;
}