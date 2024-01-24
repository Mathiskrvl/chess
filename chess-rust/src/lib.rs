use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn increment(counter: i32) -> i32 {
    counter + 1
}