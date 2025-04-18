pub mod robomap{
    pub mod drivetrain{
        pub const FR_DRIVE: i32 = 1;
        pub const FR_TURN: i32 = 2;
        pub const FR_ENCODER: i32 = 3;

        pub const FL_DRIVE: i32 = 4;
        pub const FL_TURN: i32 = 5;
        pub const FL_ENCODER: i32 = 6;

        pub const BR_DRIVE: i32 = 7;
        pub const BR_TURN: i32 = 8;
        pub const BR_ENCODER: i32 = 9;

        pub const BL_DRIVE: i32 = 10;
        pub const BL_TURN: i32 = 11;
        pub const BL_ENCODER: i32 = 12;
    }
}

pub mod joystick_map{
    pub const DRIVER_JOYSTICK_ID: i32 = 1;
}