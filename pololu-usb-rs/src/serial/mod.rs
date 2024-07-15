use std::io::{Read, Write};
use serialport::{available_ports, new, SerialPort, SerialPortBuilder};
use std::str::from_utf8;
use std::time::Duration;

pub const ADDRESS: &str = "COM6";

pub const DEVICE_NUMBER: &[u8; 1] = &[0x0c];
pub const DEVICE_BUFFER: &[u8; 1] = &[0xaa];

pub struct Maestro {
    port: Box<dyn SerialPort>,
    // targets: [u8; 24],
    // min: [u8; 24],
    // max: [u8; 24],
}

impl Maestro {
    pub fn new(path: String, baud_rate: u32) -> Self {
        let mut port: Box<dyn SerialPort> = new(path, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open Port");
        // let mut targets: [u8; 24] = [0; 24];
        // let mut min: [u8; 24] = [0; 24];
        // let mut max: [u8; 24] = [0; 24];
        Maestro {
            port,
            // targets,
            // min,
            // max,
        }
    }
    // pub fn send_cmd(&mut self, cmd: String) {
    //
    //     self.port.write(cmd_str).expect("Failed to write");
    // }
    // pub fn set_range(&mut self, idx:usize, min:u8, max:u8){
    //     self.min[idx] = min;
    //     self.max[idx] = max;
    // }
    //
    // pub fn get_min(self, idx:usize) -> u8{
    //     return self.min[idx].clone();
    // }
    //
    // pub fn get_max(&self, idx:usize) -> &u8{
    //     return &self.max[idx];
    // }
    pub fn set_speed(&mut self, idx:u8, speed:u16){
        self.port.write(&[0x84, idx, (speed & 0x7F) as u8, (speed >> 7 & 0x7F) as u8]).unwrap();
    }
    pub fn set_pos(&mut self, idx: u8, target: u16) {
        // u16: 0 -> 65535
        self.port.write(&[0x84, idx, (target & 0x7F) as u8, (target >> 7 & 0x7F) as u8]).unwrap();
    }

    pub fn get_pos(&mut self, idx:u8) -> i32 {
        self.port.write(&[0x90, idx]).unwrap();
        let mut pos_buf: &mut [u8] = &mut [0; 2];
        self.port.read(&mut pos_buf).unwrap();
        let pos: i32 = pos_buf[0] as i32 + 256 * pos_buf[1] as i32;
        return pos;
    }
}
