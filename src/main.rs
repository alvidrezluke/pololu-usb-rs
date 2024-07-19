#![warn(missing_docs)]

mod debug;
mod gui;
mod math;
mod param;
mod robot;


const FILE_PATH: &str = "description/robot_param.yml";
fn main() -> eframe::Result{
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([600.0, 500.0])
            .with_transparent(true), // To have rounded corners we need transparency

        ..Default::default()
    };
    eframe::run_native(
        "Custom window frame", // unused title
        options,
        Box::new(|_cc| Ok(Box::<gui::MyApp>::default())),
    )

}
