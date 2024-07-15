use crate::protocols::UscParameter;

pub const DeviceInterfaceGuid: &str = "e0fbe39f-7670-4db6-9b1a-1dfb141014a7";
pub const VendorId: u32 = 0x1ffb;

pub const ProductIds: [u8; 4] = [0x0089, 0x008a, 0x008b, 0x008c];

pub const EnglishName: &str = "Maestro USB servo controller";

pub const InstructionFrequency: u32 = 12000000;

pub const BootloaderDeviceInstanceIdPrefixes: [&str; 4] = [
    "USB\\VID_1FFB&PID_0088",
    "USB\\VID_1FFB&PID_008D",
    "USB\\VID_1FFB&PID_008E",
    "USB\\VID_1FFB&PID_008F",
];

pub const ShortProductName: &str = "Maestro";

pub fn exponential_speed_to_normal_speed(exponential_speed: u8) -> u32 {
    let mantissa: u8 = exponential_speed >> 3;
    let exponent: u8 = exponential_speed & 7;
    return (mantissa * (1 << exponent)) as u32;
}

pub fn normal_speed_to_exponential_speed(normal_speed: u32) -> u8 {
    let mut mantissa: u32 = normal_speed;
    let mut exponent: u32 = 0;
    loop {
        if mantissa < 32 {
            return (exponent + (mantissa << 3)) as u8;
        }

        if exponent == 7 {
            return 0xFF;
        }

        exponent += 1;
        mantissa >>= 1;
    }
}

pub fn position_to_microseconds(position: u32) -> f32 {
    return position as f32 / 4.0;
}

pub fn microseconds_to_position(microseconds: f32) -> u32 {
    return (microseconds * 4.0) as u32;
}

// Note u32 only applies for mini_maestro_servo_period (changed from u32 to u8) (might have to change back later)
pub fn period_to_microseconds(period: u8, servos_available: u8) -> f32 {
    return period as f32 * 256.0 * servos_available as f32 / 12.0;
}

pub fn microseconds_to_period(microseconds: f32, servos_available: u8) -> f32 {
    return (microseconds / 256.0 * 12.0 / servos_available as f32).round();
}

pub fn convert_spbrg_to_bps(spbrg: u16) -> u32 {
    if spbrg == 0 {
        return 0;
    }
    return ((InstructionFrequency + (spbrg as u32 + 1) / 2) / (spbrg as u32 + 1));
}

pub fn convert_bps_to_spbrg(bps: u32) -> u16 {
    if bps == 0 {
        return 0;
    }
    return ((InstructionFrequency - bps / 2) / bps) as u16;
}

pub fn channel_to_port(channel: u8) -> u8 {
    if channel <= 3 {
        return channel;
    } else if channel < 6 {
        return (channel + 2) as u8;
    }
    panic!("Invalid channel number: {}", channel);
}
pub const ServoParameterBytes: u8 = 9;
pub fn specify_servo(p: UscParameter, servo: u8) -> UscParameter {
    return (p as u8 + servo + ServoParameterBytes).try_into().unwrap();
}

pub struct Usc {
    servo_count: u8,
    stack_size: u8,
    call_stack_size: u8,
    driver_type: MaestroType,
}

enum MaestroType {
    MicroMaestro,
    MiniMaestro,
}

pub const MICRO_MAESTRO_STACK_SIZE: u8 = 32;
pub const MICRO_MAESTRO_CALL_STACK_SIZE: u8 = 10;

pub const MINI_MAESTRO_STACK_SIZE: u8 = 126;
pub const MINI_MAESTRO_CALL_STACK_SIZE: u8 = 126;

// impl Usc {
//     pub fn new(device: nusb::Device) -> Self {
//         let mut servo_count = 0;
//         match device.product_id() {
//             0x89 => servo_count = 6,
//             0x8a => servo_count = 12,
//             0x8b => servo_count = 18,
//             0x8c => servo_count = 24,
//             _ => panic!("Unknown product id: {}.", device.get_product_id())
//         }
//         let mut stack_size = MINI_MAESTRO_STACK_SIZE;
//         let mut call_stack_size = MINI_MAESTRO_CALL_STACK_SIZE;
//         let mut driver_type = MaestroType::MiniMaestro;
//         if servo_count == 6 {
//             stack_size = MICRO_MAESTRO_STACK_SIZE;
//             call_stack_size = MICRO_MAESTRO_CALL_STACK_SIZE;
//             driver_type = MaestroType::MicroMaestro;
//         }
//
//         let firmware_major: u8 = 0xFF;
//         let firmware_minor: u8 = 0xFF;
//         if firmware_major == 0xFF {
//             // self::get_firmware_version()
//         }
//
//         Usc {
//             servo_count,
//             stack_size,
//             call_stack_size,
//             driver_type,
//             //handler
//         }
//     }
//
//     fn get_firmware_version(&self) -> (u8, u8) {
//         let mut buffer: [u8; 14] = [0;14];
//         // controlTransfer(0x80, 6, 0x0100, 0x0000, buffer).unwrap();
//         let firmwware_minor = ((buffer[12] & 0xF) + (buffer[12] >> 4 & 0xF) * 10);
//         let firmware_major = ((buffer[13] & 0xF) + (buffer[13] >> 4 & 0xF) * 10);
//         todo!()
//     }
// }
