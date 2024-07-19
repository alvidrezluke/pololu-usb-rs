mod helper;
mod frame_window;

use eframe;
use egui::Ui;
use crate::gui::helper::format_data;

#[derive(Clone, Debug)]
struct Coordinate(&'static str, f64);
#[derive(Clone, Debug)]
enum DisplayMode {
    Platform,
    Servo,
}
#[derive(Clone, Debug)]
pub(crate) struct MyApp {
    display_mode: DisplayMode,
    target_pos: [Coordinate; 3],
    orientation: [Coordinate; 3],
}
impl Default for MyApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        frame_window::custom_window_frame(ctx, "Robot GUI", |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Type of Robot: Stewart Platform");
            });
            self.display_data(ui);
        });

    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }
}
impl MyApp {
    pub fn new() -> Self {
        Self{
            display_mode: DisplayMode::Platform,
            target_pos: [Coordinate("X", 0.0),Coordinate("Y", 0.0),Coordinate("Z", 0.0)],
            orientation: [Coordinate("Roll", 0.0),Coordinate("Pitch", 0.0), Coordinate("Yaw", 0.0)],
        }
    }
    fn display_data(&mut self, ui: &mut Ui){
        ui.separator();
        ui.heading("Platform Position");
        egui::Grid::new("Platform Position")
            .num_columns(2)
            .spacing([20.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                format_data(ui, &mut self.target_pos);
            });
        ui.separator();
        ui.heading("Platform Orientation");
        egui::Grid::new("Platform Orientation")
            .num_columns(2)
            .spacing([20.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                format_data(ui, &mut self.orientation);
            });
    }
}


