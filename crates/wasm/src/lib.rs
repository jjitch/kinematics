use kinematics_core::hello_message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    hello_message()
}
