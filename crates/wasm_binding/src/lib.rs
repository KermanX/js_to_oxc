use js_to_oxc::js_to_oxc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(s: &str) -> String {
    js_to_oxc(s)
}
