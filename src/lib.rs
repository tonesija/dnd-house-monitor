mod utils;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    selected_entity_id: Option<String>,
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
            selected_entity_id: None,
            entities: Vec::new(),
        }
    }

    pub fn entities(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.entities).unwrap()
    }

    pub fn add_entity(&mut self, id: &str, is_player: bool) {
        let new_entity = Entity {
            id: String::from_str(id).unwrap(),
            is_player: is_player,
            is_placed: false,
            x: 0,
            y: 0,
            initiative: 0,
        };
        self.entities.push(new_entity);
        self.sort_entities();
    }

    pub fn set_initiative(&mut self, id: &str, initiative: u32) {
        if let Some(idx) = self.entities.iter().position(|e| e.id == id) {
            self.entities[idx].initiative = initiative;
            self.sort_entities();
        }
    }

    pub fn next(&mut self) -> JsValue {
        match &self.selected_entity_id {
            None => {
                self.selected_entity_id = Some(String::from_str(&self.entities[0].id).unwrap());
            }
            Some(selected_entity_id) => {
                let idx = self
                    .entities
                    .iter()
                    .position(|e| e.id.eq(selected_entity_id))
                    .unwrap();
                if let Some(e) = self.entities.get(idx + 1) {
                    self.selected_entity_id = Some(String::from_str(&e.id).unwrap());
                } else {
                    self.selected_entity_id = Some(String::from_str(&self.entities[0].id).unwrap());
                }
            }
        }

        serde_wasm_bindgen::to_value(&self.selected_entity_id).unwrap()
    }

    pub fn place(&mut self, id: &str, x: u32, y: u32) {
        if let Some(idx) = self.entities.iter().position(|e| e.id == id) {
            self.entities[idx].is_placed = true;
            self.entities[idx].x = x;
            self.entities[idx].y = y;
        }
    }
}

impl Map {
    fn sort_entities(&mut self) {
        self.entities
            .sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }
}

#[derive(Serialize, Deserialize)]
struct Entity {
    id: String,
    initiative: u32,
    is_player: bool,
    is_placed: bool,
    x: u32,
    y: u32,
}

impl Entity {}
