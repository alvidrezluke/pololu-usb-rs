use pololu_usb_rs::serial;
fn main() {
    let path: &str = "COM6";
    let baud_rate: u32 = 9600;

    let mut dev = serial::Maestro::new(path.into(), baud_rate);
    let pos = dev.get_pos(0);

    println!("Pos: {}", pos);
    dev.set_pos(0, 65535);
    let pos = dev.get_pos(0);

    println!("Pos: {}", pos);
}
