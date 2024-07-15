use crate::maestro;
use crate::protocols::UscSerialMode;

pub const SERVO_PERIOD: u8 = 156;
pub const SERVOS_AVAILABLE: u8 = 6;
pub const MINI_MAESTRO_SERVO_PERIOD: u32 = 80000;
pub const FIXED_BAUD_RATE: u32 = 9600;
pub const SERIAL_DEVICE_NUMBER: u8 = 12;

#[derive(Debug)]
pub enum ChannelMode {
    Servo = 0,
    ServoMultiplied = 1,
    Output = 2,
    Input = 3,
}

#[derive(Debug)]
pub enum HomeMode {
    Off,
    Ignore,
    Goto,
}

#[derive(Debug)]
pub struct ChannelSetting {
    name: String,
    channel_mode: ChannelMode,
    home_mode: HomeMode,
    home: u16,
    min: u16,
    max: u16,
    neutral: u16,
    range: u16,
    speed: u16,
    accel: u16,
}
#[derive(Debug)]
pub struct UscSettings {
    servos_available: u8,
    servo_period: u8,
    mini_maestro_servo_period: u32,
    servo_multiplier: u16,
    serial_mode: UscSerialMode,
    fixed_baud_rate: u32,
    enable_crc: bool,
    never_suspend: bool,
    serial_device_number: u8,
    mini_ssc_offset: u8,
    serial_timeout: u8,
    script_done: bool,
    channel_settings: Vec<ChannelSetting>,
    enable_pullups: bool,
    script_inconsistent: bool,
    private_script: String,
    servo_count: u8,
    // private_Program: BytecodeProgram ???
}

// IDK ANYMORE
impl UscSettings {
    pub fn new() -> Self {
        let mut channel_settings: Vec<ChannelSetting> = vec![];
        let mut servo_count: u8 = channel_settings.len() as u8;
        // Kept as f32
        let mut period_in_microseconds: f32 = 0.0;

        if servo_count == 6 {
            period_in_microseconds =
                maestro::period_to_microseconds(SERVO_PERIOD, SERVOS_AVAILABLE);
        } else {
            period_in_microseconds = (MINI_MAESTRO_SERVO_PERIOD / 4) as f32;
        }
        return UscSettings {
            servos_available: SERVOS_AVAILABLE,
            servo_period: SERVO_PERIOD,
            mini_maestro_servo_period: MINI_MAESTRO_SERVO_PERIOD,
            servo_multiplier: 1,
            serial_mode: UscSerialMode::SerialModeUartDetectBaudRate,
            fixed_baud_rate: FIXED_BAUD_RATE,
            enable_crc: false,
            never_suspend: false,
            serial_device_number: SERIAL_DEVICE_NUMBER,
            mini_ssc_offset: 0,
            serial_timeout: 0,
            script_done: true,
            channel_settings,
            enable_pullups: false,
            script_inconsistent: false,
            private_script: "".to_string(),
            servo_count,
            // private_Program: BytecodeProgram,
        };
    }
    // pub fn setAndCompileScript(string script)
}
