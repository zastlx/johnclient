use egui_keybinds::KeyBind;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use egui::*;
use egui_keybinds::*;
use enigo::*;
use key_names::*;
use lazy_static::lazy_static;
use mki::*;
use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Error;
use std::sync::{Arc, LockResult, Mutex, RwLock};
use std::thread;

use super::structs;

lazy_static! {
    static ref CONF: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref VALS: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref BINDS: Arc<Mutex<HashMap<String, KeyBind>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub trait LockResultExt {
    type Guard;
    fn ignore_poison(self) -> Self::Guard;
}

impl<Guard> LockResultExt for LockResult<Guard> {
    type Guard = Guard;

    fn ignore_poison(self) -> Guard {
        self.unwrap_or_else(|e| e.into_inner())
    }
}

pub fn render_ui(_self: &mut structs::App, ui: &mut Ui) {
    ui.group(|g| {
        g.horizontal(|h| {
            h.colored_label(Color32::from_rgb(120, 81, 169), "Left");
            h.checkbox(&mut _self.left, "");
            h.add(KeyBindWidget::new(&mut _self.bind.left.as_mut().unwrap()));
        });
        g.vertical_centered(|v| {
            v.add(egui::widgets::Slider::new(&mut _self.left_min, 0..=100).text("Min"));
            v.add(egui::widgets::Slider::new(&mut _self.left_max, 0..=100).text("Max"));
        });
    });
    ui.group(|g| {
        g.horizontal(|h| {
            h.colored_label(Color32::from_rgb(120, 81, 169), "Right");
            h.checkbox(&mut _self.right, "");
            h.add(KeyBindWidget::new(&mut _self.bind.right.as_mut().unwrap()));
        });
        g.vertical_centered(|v| {
            v.add(egui::widgets::Slider::new(&mut _self.right_min, 0..=100).text("Min"));
            v.add(egui::widgets::Slider::new(&mut _self.right_max, 0..=100).text("Max"));
        });
    });
    ui.group(|g| {
        g.horizontal(|h| {
            h.colored_label(Color32::from_rgb(120, 81, 169), "Block Hit");
            h.checkbox(&mut _self.block_hit, "");
            h.add(KeyBindWidget::new(
                &mut _self.bind.block_hit.as_mut().unwrap(),
            ));
        });
        g.vertical_centered(|v| {
            v.add(egui::widgets::Slider::new(&mut _self.block_hit_min, 0..=100).text("Min"));
            v.add(egui::widgets::Slider::new(&mut _self.block_hit_max, 0..=100).text("Max"));
        });
    });
    ui.separator();
    ui.group(|g| {
        g.horizontal(|h| {
            h.colored_label(Color32::from_rgb(120, 81, 169), "Jitter");
            h.add(KeyBindWidget::new(
                &mut _self.bind.block_hit.as_mut().unwrap(),
            ));
        });
        g.vertical_centered(|v| {
            v.add(egui::widgets::Slider::new(&mut _self.jitter_x, 0..=100).text("X"));
            v.add(egui::widgets::Slider::new(&mut _self.jitter_y, 0..=100).text("Y"));
        });
    });
    ui.separator();
    if ui.button("Destruct").clicked() {
        let mut binds = BINDS.lock().ignore_poison();
        binds.clear();
        let mut conf = CONF.lock().ignore_poison();
        conf.clear();
        let mut vals = VALS.lock().ignore_poison();
        vals.clear();
        drop(_self);
        std::process::exit(0);
        return;
    }
    let mut binds = BINDS.lock().ignore_poison();
    binds.insert(
        "left".to_string(),
        Some(_self.bind.left.as_mut().unwrap())
            .as_deref()
            .unwrap()
            .to_owned(),
    );
    binds.insert(
        "right".to_string(),
        Some(_self.bind.right.as_mut().unwrap())
            .as_deref()
            .unwrap()
            .to_owned(),
    );
    binds.insert(
        "block_hit".to_string(),
        Some(_self.bind.block_hit.as_mut().unwrap())
            .as_deref()
            .unwrap()
            .to_owned(),
    );
    let mut conf = CONF.lock().ignore_poison();
    conf.insert("left".to_string(), _self.left);
    conf.insert("right".to_string(), _self.right);
    conf.insert("block_hit".to_string(), _self.block_hit);
    let mut vals = VALS.lock().ignore_poison();
    vals.insert(
        "left_min".to_string(),
        if _self.left_min == 0 {
            4
        } else {
            _self.left_min
        },
    );
    vals.insert(
        "left_max".to_string(),
        if _self.left_max == 0 {
            12
        } else {
            _self.left_max
        },
    );
    vals.insert(
        "right_min".to_string(),
        if _self.right_min == 0 {
            5
        } else {
            _self.left_min
        },
    );
    vals.insert(
        "right_max".to_string(),
        if _self.right_max == 0 {
            16
        } else {
            _self.left_min
        },
    );
    vals.insert(
        "block_hit_min".to_string(),
        if _self.block_hit_min == 0 {
            10
        } else {
            _self.block_hit_min
        },
    );
    vals.insert(
        "block_hit_max".to_string(),
        if _self.block_hit_max == 0 {
            15
        } else {
            _self.block_hit_max
        },
    );
    vals.insert(
        "jitter_x".to_string(),
        if _self.jitter_x == 0 {
            4
        } else {
            _self.jitter_x
        },
    );
    vals.insert(
        "jitter_y".to_string(),
        if _self.jitter_y == 0 {
            10
        } else {
            _self.jitter_y
        },
    );
}

pub fn hook_reg() {
    mki::bind_any_key(mki::Action::handle_kb(|event| {
        let mut enigo = Enigo::new();
        let mut locked_v = VALS.lock().ignore_poison();
        let locked_c = CONF.lock().ignore_poison();
        let locked_b = BINDS.lock().ignore_poison();
        let _vals_obj = locked_v;
        let _conf_obj = locked_c;
        let _binds_obj = locked_b;
        let left = _conf_obj.get("left").unwrap();
        let bindl = _binds_obj.get("left").unwrap();
        let bindr = _binds_obj.get("right").unwrap();
        let bindb = _binds_obj.get("block_hit").unwrap();
        let kb = from_str(bindl.clone().serialize().as_str()).unwrap();
        let kb2 = from_str(bindr.clone().serialize().as_str()).unwrap();
        let kb3 = from_str(bindb.clone().serialize().as_str()).unwrap();
        drop(_conf_obj);
        drop(_binds_obj);
        if kb.is_pressed() {
            while kb.is_pressed() {
                let min_delay = _vals_obj.get("left_min").unwrap();
                let max_delay = _vals_obj.get("left_max").unwrap();
                let delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(delay as u64));
                let mut x = *_vals_obj.get("jitter_x").unwrap();
                let mut y = *_vals_obj.get("jitter_y").unwrap();
                let mut jitter_x = rand::thread_rng().gen_range(-x..x);
                let mut jitter_y = rand::thread_rng().gen_range(-y..y);
                enigo.mouse_move_relative(jitter_x, jitter_y);
                enigo.mouse_down(MouseButton::Left);
                let up_delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(up_delay as u64));
                enigo.mouse_up(MouseButton::Left);
            }
        } else if kb2.is_pressed() {
            while kb2.is_pressed() {
                let min_delay = _vals_obj.get("left_min").unwrap();
                let max_delay = _vals_obj.get("left_max").unwrap();
                let delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(delay as u64));
                let mut x = *_vals_obj.get("jitter_x").unwrap();
                let mut y = *_vals_obj.get("jitter_y").unwrap();
                let mut jitter_x = rand::thread_rng().gen_range(-x..x);
                let mut jitter_y = rand::thread_rng().gen_range(-y..y);
                enigo.mouse_move_relative(jitter_x, jitter_y);
                enigo.mouse_down(MouseButton::Right);
                let up_delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(up_delay as u64));
                enigo.mouse_up(MouseButton::Right);
            }
        } else if kb3.is_pressed() {
            while kb3.is_pressed() {
                let min_delay = _vals_obj.get("block_hit_min").unwrap();
                let max_delay = _vals_obj.get("block_hit_max").unwrap();
                let delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(delay as u64));
                let mut x = *_vals_obj.get("jitter_x").unwrap();
                let mut y = *_vals_obj.get("jitter_y").unwrap();
                let mut jitter_x = rand::thread_rng().gen_range(-x..x);
                let mut jitter_y = rand::thread_rng().gen_range(-y..y);
                enigo.mouse_move_relative(jitter_x, jitter_y);
                enigo.mouse_down(MouseButton::Left);
                let up_delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(up_delay as u64));
                enigo.mouse_up(MouseButton::Left);
                let block_delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(block_delay as u64));
                enigo.mouse_down(MouseButton::Right);
                let block_up_delay =
                    rand::thread_rng().gen_range(min_delay.to_owned()..max_delay.to_owned());
                thread::sleep(std::time::Duration::from_millis(block_up_delay as u64));
                enigo.mouse_up(MouseButton::Right);
            }
        }
    }));
}

fn from_str(s: &str) -> Result<Keyboard, Error> {
    let parsed = match s {
        "A" => Keyboard::A,
        "B" => Keyboard::B,
        "C" => Keyboard::C,
        "D" => Keyboard::D,
        "E" => Keyboard::E,
        "F" => Keyboard::F,
        "G" => Keyboard::G,
        "H" => Keyboard::H,
        "I" => Keyboard::I,
        "J" => Keyboard::J,
        "K" => Keyboard::K,
        "L" => Keyboard::L,
        "M" => Keyboard::M,
        "N" => Keyboard::N,
        "O" => Keyboard::O,
        "P" => Keyboard::P,
        "Q" => Keyboard::Q,
        "R" => Keyboard::R,
        "S" => Keyboard::S,
        "T" => Keyboard::T,
        "U" => Keyboard::U,
        "V" => Keyboard::V,
        "W" => Keyboard::W,
        "X" => Keyboard::X,
        "Y" => Keyboard::Y,
        "Z" => Keyboard::Z,
        "0" => Keyboard::Number0,
        "1" => Keyboard::Number1,
        "2" => Keyboard::Number2,
        "3" => Keyboard::Number3,
        "4" => Keyboard::Number4,
        "5" => Keyboard::Number5,
        "6" => Keyboard::Number6,
        "7" => Keyboard::Number7,
        "8" => Keyboard::Number8,
        "9" => Keyboard::Number9,
        "LeftAlt" => Keyboard::LeftAlt,
        "RightAlt" => Keyboard::RightAlt,
        "LeftShift" => Keyboard::LeftShift,
        "RightShift" => Keyboard::RightShift,
        "LeftControl" => Keyboard::LeftControl,
        "RightControl" => Keyboard::RightControl,
        "BackSpace" => Keyboard::BackSpace,
        "Tab" | "	" => Keyboard::Tab,
        "Enter" | "\n" => Keyboard::Enter,
        "Escape" => Keyboard::Escape,
        "Space" | " " => Keyboard::Space,
        "PageUp" => Keyboard::PageUp,
        "PageDown" => Keyboard::PageDown,
        "Home" => Keyboard::Home,
        "Left" => Keyboard::Left,
        "Up" => Keyboard::Up,
        "Right" => Keyboard::Right,
        "Down" => Keyboard::Down,
        "Print" => Keyboard::Print,
        "PrintScreen" => Keyboard::PrintScreen,
        "Insert" => Keyboard::Insert,
        "Delete" => Keyboard::Delete,
        "LeftWindows" => Keyboard::LeftWindows,
        "RightWindows" => Keyboard::RightWindows,
        "Comma" | "," => Keyboard::Comma,
        "Period" | "." => Keyboard::Period,
        "Slash" | "/" => Keyboard::Slash,
        "SemiColon" | ";" | ":" => Keyboard::SemiColon,
        "Apostrophe" | "'" | "\"" => Keyboard::Apostrophe,
        "LeftBrace" | "[" => Keyboard::LeftBrace,
        "BackwardSlash" | "\\" => Keyboard::BackwardSlash,
        "RightBrace" | "]" => Keyboard::RightBrace,
        "Grave" | "`" => Keyboard::Grave,
        "F1" => Keyboard::F1,
        "F2" => Keyboard::F2,
        "F3" => Keyboard::F3,
        "F4" => Keyboard::F4,
        "F5" => Keyboard::F5,
        "F6" => Keyboard::F6,
        "F7" => Keyboard::F7,
        "F8" => Keyboard::F8,
        "F9" => Keyboard::F9,
        "F10" => Keyboard::F10,
        "F11" => Keyboard::F11,
        "F12" => Keyboard::F12,
        "F13" => Keyboard::F13,
        "F14" => Keyboard::F14,
        "F15" => Keyboard::F15,
        "F16" => Keyboard::F16,
        "F17" => Keyboard::F17,
        "F18" => Keyboard::F18,
        "F19" => Keyboard::F19,
        "F20" => Keyboard::F20,
        "F21" => Keyboard::F21,
        "F22" => Keyboard::F22,
        "F23" => Keyboard::F23,
        "F24" => Keyboard::F24,
        "NumLock" => Keyboard::NumLock,
        "ScrollLock" => Keyboard::ScrollLock,
        "CapsLock" => Keyboard::CapsLock,
        "Numpad0" => Keyboard::Numpad0,
        "Numpad1" => Keyboard::Numpad1,
        "Numpad2" => Keyboard::Numpad2,
        "Numpad3" => Keyboard::Numpad3,
        "Numpad4" => Keyboard::Numpad4,
        "Numpad5" => Keyboard::Numpad5,
        "Numpad6" => Keyboard::Numpad6,
        "Numpad7" => Keyboard::Numpad7,
        "Numpad8" => Keyboard::Numpad8,
        "Numpad9" => Keyboard::Numpad9,
        "Multiply" => Keyboard::Multiply,
        "Add" => Keyboard::Add,
        "Separator" => Keyboard::Separator,
        "Subtract" => Keyboard::Subtract,
        "Decimal" => Keyboard::Decimal,
        "Divide" => Keyboard::Divide,
        _ => Keyboard::Other(0),
    };
    Ok(parsed)
}
