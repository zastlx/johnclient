
use super::clicker;
use super::ui;

#[derive(Default)]
pub struct App {
    pub left: bool,
    pub right: bool,
    pub block_hit: bool,
    pub left_min: i32,
    pub left_max: i32,
    pub right_min: i32,
    pub right_max: i32,
    pub block_hit_min: i32,
    pub block_hit_max: i32,
    pub jitter_x: i32,
    pub jitter_y: i32,
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ui::custom_window_frame(ctx, frame, "egui with custom frame", |ui| {
            clicker::render_ui(self, ui);
        });
    }
}