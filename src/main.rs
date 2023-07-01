#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mods;

fn main() -> Result<(), eframe::Error> {
    mods::clicker::register_hooks();

    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(400.0, 100.0)),
        initial_window_size: Some(egui::vec2(300.0, 400.0)),
        initial_window_pos: Some(egui::pos2(0.0, 0.0)),
        always_on_top: true,
        ..Default::default()
    };
    eframe::run_native(
        &*mods::utils::get_name(),
        options,
        Box::new(|_cc| Box::<mods::structs::App>::default()),
    )
}