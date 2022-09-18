mod utils;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    entities: Vec<Entity>,
}
#[wasm_bindgen]
impl Map {
    pub fn new() -> Map {
        let height = 24;
        let width = 32;
        Map {
            width: width,
            height: height,
            entities: Vec::new(),
        }
    }

    pub fn entities(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.entities).unwrap()
    }

    pub fn add_entity(&mut self, id: &str) {
        let new_entity = Entity {
            id: String::from_str(id).unwrap(),
            initiative: 0,
        };
        self.entities.push(new_entity);
    }

    pub fn set_initiative(&mut self, id: &str, initiative: u32) {
        if let Some(idx) = self.entities.iter().position(|e| e.id == id) {
            self.entities[idx].initiative = initiative;
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Entity {
    id: String,
    initiative: u32,
}

impl Entity {}
