pub mod poem;
pub use poem::*; 

pub mod ui;
pub use ui::*;

pub mod wasm;   
pub use wasm::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("Hello from WASM!")
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

