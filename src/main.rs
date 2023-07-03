#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image;

mod mods;

fn load_icon() -> eframe::IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../including/icon.png");
		let image = image::load_from_memory(icon)
			.expect("Failed to open icon path")
			.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	
	eframe::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}


fn main() -> Result<(), eframe::Error> {
    mods::clicker::hook_reg();
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(400.0, 100.0)),
        initial_window_size: Some(egui::vec2(350.0, 500.0)),
        initial_window_pos: Some(egui::pos2(0.0, 0.0)),
        always_on_top: true,
        icon_data: Some(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        &*mods::utils::get_name(),
        options,
        Box::new(|_cc| Box::<mods::structs::App>::default()),
    )
}