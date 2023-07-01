use egui::*;
use mki::*;
use std::sync::{Mutex, Arc};
use std::thread;
use std::collections::HashMap;
use rand::Rng;
use lazy_static::lazy_static;

use super::structs;


lazy_static! {
    static ref CONF: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref VALS: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn render_ui(_self: &mut structs::App, ui: &mut Ui) {
    ui.group(|g| {
        g.horizontal(|h| {
            h.colored_label(Color32::from_rgb(120, 81, 169), "Left");
            h.checkbox(&mut _self.left, "");
        });
        g.vertical_centered(|v| {
            v.add(egui::widgets::Slider::new(&mut _self.left_min, 0..=100).text("Min"));
            v.add(egui::widgets::Slider::new(&mut _self.left_max, 0..=100).text("Max"));
        });
    });

    let mut conf = CONF.lock().unwrap();
    conf.insert("left".to_string(), _self.left);
    conf.insert("right".to_string(), _self.right);
    conf.insert("block_hit".to_string(), _self.block_hit);
    let mut vals = VALS.lock().unwrap();
    vals.insert("left_min".to_string(), _self.left_min);
    vals.insert("left_max".to_string(), _self.left_max);
    vals.insert("right_min".to_string(), _self.right_min);
    vals.insert("right_max".to_string(), _self.right_max);
    vals.insert("block_hit_min".to_string(), _self.block_hit_min);
    vals.insert("block_hit_max".to_string(), _self.block_hit_max);
    vals.insert("jitter_x".to_string(), _self.jitter_x);
    vals.insert("jitter_y".to_string(), _self.jitter_y);
}

pub fn exec_left_down(event: Mouse) {
    let mut locked_v = VALS.lock();
    let mut locked_c = CONF.lock();
    let mut _vals_obj = locked_v.as_mut().unwrap();
    let mut _conf_obj = locked_c.as_mut().unwrap();
    if _conf_obj.get("left") == Some(&true) {
        let min = _vals_obj.get("left_min").unwrap();
        let max = _vals_obj.get("left_max").unwrap();
        while event.is_pressed() {
            let mut rng = rand::thread_rng();
            let c_delay = rng.gen_range(*min..*max);
            thread::sleep(std::time::Duration::from_millis(c_delay as u64));
            event.click();
            let mut jitter = _vals_obj.get("jitter_x").unwrap().to_owned();
            let mut jitter_y = _vals_obj.get("jitter_y").unwrap().to_owned();
            let x = rng.gen_range(-jitter.to_owned()..jitter.to_owned());
            let y = rng.gen_range(-jitter_y.to_owned()..jitter_y.to_owned());
            Mouse::move_by(x, y);
        }
    }
}

pub fn register_hooks() {
    Mouse::Left.bind(|event| { exec_left_down(event) });
}