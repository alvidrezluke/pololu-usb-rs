use eframe::egui::{self};
use crate::gui::Coordinate;

pub fn format_data(ui: &mut egui::Ui, data: &mut [Coordinate; 3]){
    for mut ax in data {
        ui.label(ax.0);
        ui.add(egui::Slider::new(&mut ax.1, 0.0..=120.0));
        ui.end_row();
    }
}

