use crate::protocols::UscSerialMode;

pub enum ChannelMode {
    Servo = 0,
    ServoMultiplied = 1,
    Output = 2,
    Input = 3,
}

pub enum HomeMode
{
    Off,
    Ignore,
    Goto
}

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
    accel: u16
}
pub struct UscSettings{
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
    // privateProgram:
}

impl Default for UscSettings{
    fn default() -> Self {

    }
}




// pub const SERVOS_AVAILABLE:u8 = 6;
// pub const SERVO_PERIOD:u8 = 156;

