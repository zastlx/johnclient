use egui::*;
use mki::*;
use std::cell::RefCell;
use std::thread;
use std::collections::HashMap;
use rand::Rng;

use super::structs;


thread_local! {
    static CONF: RefCell<HashMap<String, bool>> = RefCell::new(HashMap::new());
    static VALS: RefCell<HashMap<String, i32>> = RefCell::new(HashMap::new());
}

pub fn render_ui(_self: &mut structs::App, ui: &mut Ui) {
    CONF.with(|c| {
        let mut conf = c.borrow_mut();
        conf.insert("left".to_string(), _self.left);
        conf.insert("right".to_string(), _self.right);
        conf.insert("block_hit".to_string(), _self.block_hit);
    });
    VALS.with(|v| {
        let mut vals = v.borrow_mut();
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

pub fn register_hooks() {
    Mouse::Left.bind(|event| {
        VALS.with(|v| {
            let vals = v.borrow_mut();
            CONF.with(|c| {
                if c.borrow().get("left").unwrap() == &true {
                    let min = vals.get("left_min").unwrap();
                    let max = vals.get("left_max").unwrap();
                    while event.is_pressed() {
                        let mut rng = rand::thread_rng();
                        let c_delay = rng.gen_range(*min..*max);
                        thread::sleep(std::time::Duration::from_millis(c_delay as u64));
                        event.click();
                        let jitter = vals.get("jitter_x").unwrap();
                        let jitter_y = vals.get("jitter_y").unwrap();
                        let x = rng.gen_range(-*jitter..*jitter);
                        let y = rng.gen_range(-*jitter_y..*jitter_y);
                        Mouse::move_by(x, y);
                    }
                }
            });
        });
    });
}