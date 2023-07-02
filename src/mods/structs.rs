use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::Display;
use egui_keybinds::*;

use super::clicker;
use super::ui;
use super::utils;

use egui::{Key, Modifiers};


type Binding = Option<KeyBind>;

#[derive(Default)]
pub struct Bindings {
    pub left: Binding,
    pub right: Binding,
    pub block_hit: Binding,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            left: Some(KeyBind::new(Some(KeyCode::L), vec![])),
            right: Some(KeyBind::new(Some(KeyCode::R), vec![])),
            block_hit: Some(KeyBind::new(Some(KeyCode::H), vec![])),
        }
    }
}

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

    pub bind: Bindings
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.bind.left.is_none() {
            self.bind = Bindings::new();
        }

        ui::custom_window_frame(ctx, frame, utils::get_name().as_str(), |ui| {
            clicker::render_ui(self, ui);
        });
    }
}