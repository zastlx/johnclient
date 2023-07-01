use egui::*;
use mki::*;
use std::sync::{Mutex, Arc};
use std::thread;
use std::collections::HashMap;
use rand::Rng;

use super::structs;


thread_local! {
    static CONF: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    static VALS: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn render_ui(_self: &mut structs::App, ui: &mut Ui) {
    CONF.with(|c| {
        let mut conf = c.lock().unwrap();
        conf.insert("left".to_string(), _self.left);
        conf.insert("right".to_string(), _self.right);
        conf.insert("block_hit".to_string(), _self.block_hit);
    });
    VALS.with(|v| {
        let mut vals = v.lock().unwrap();
        vals.insert("left_min".to_string(), _self.left_min);
        vals.insert("left_max".to_string(), _self.left_max);
        vals.insert("right_min".to_string(), _self.right_min);
        vals.insert("right_max".to_string(), _self.right_max);
        vals.insert("block_hit_min".to_string(), _self.block_hit_min);
        vals.insert("block_hit_max".to_string(), _self.block_hit_max);
        vals.insert("jitter_x".to_string(), _self.jitter_x);
        vals.insert("jitter_y".to_string(), _self.jitter_y);
    });
}

pub fn exec_left_down(event: Mouse) {
    let mut _vals_obj = VALS.with(|v| {
        let vals = v.lock().unwrap();
        vals.clone()
    });
    let mut _conf_obj = CONF.with(|c| {
        let conf = c.lock().unwrap();
        conf.clone()
    });
    if _conf_obj.get("left") == Some(&true) {
        let min = _vals_obj.get("left_min").unwrap();
        let max = _vals_obj.get("left_max").unwrap();
        while event.is_pressed() {
            let mut rng = rand::thread_rng();
            let c_delay = rng.gen_range(*min..*max);
            thread::sleep(std::time::Duration::from_millis(c_delay as u64));
            event.click();
            let jitter = _vals_obj.get("jitter_x").unwrap();
            let jitter_y = _vals_obj.get("jitter_y").unwrap();
            let x = rng.gen_range(-*jitter..*jitter);
            let y = rng.gen_range(-*jitter_y..*jitter_y);
            Mouse::move_by(x, y);
        }
    }
}

pub fn register_hooks() {
    Mouse::Left.bind(|event| { exec_left_down(event) });
}