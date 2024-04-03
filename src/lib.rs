mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(msg: String) -> String {
  msg
}
