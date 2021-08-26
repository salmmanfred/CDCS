use crate::ui_ext::popups::ask;
use crate::ui_ext::popups::note;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::common_traits::*;

const SAVE_FILE: &str = "map.json";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct prov {
    pub provinces: HashMap<u32, String>,
    provinces_reverse: HashMap<String, u32>,
}
impl prov {
    pub fn new() -> Self {
        if Path::new(SAVE_FILE).exists() {
            let x: Self = serde_json::from_str(&openfile::read_file(SAVE_FILE)).unwrap_e("err");
            return x;
        }
        Self {
            provinces: HashMap::new(),
            provinces_reverse: HashMap::new(),
        }
    }

    pub fn add(&mut self, color: u32, name: String) {
        if self.provinces.get(&color) == Some(&name) {
            return; // Already registered the same way
        }
        if self.provinces_reverse.contains_key(&name) {
            // Make sure not to have two provinces with the same name
            note::note("Province name already exist. Not writing");
        } else if self.provinces.contains_key(&color) {
            if ask::ask("Province color already registred. Write over?") {
                let prev_name = self.provinces.get(&color).unwrap();
                self.provinces_reverse.remove(prev_name);
                self.provinces.insert(color, name.clone());
                self.provinces_reverse.insert(name, color);
            }
        } else {
            self.provinces.insert(color, name.clone());
            self.provinces_reverse.insert(name, color);
        }
    }
    pub fn save(&mut self) {
        openfile::write_file(SAVE_FILE, &serde_json::to_string(&self).unwrap_e("err"))
            .unwrap_e("err");
    }
    pub fn exists(&self, colour: u32) -> bool {
        match self.provinces.get(&colour) {
            Some(_) => true, //return (),
            _ => false,
        }
    }
}
