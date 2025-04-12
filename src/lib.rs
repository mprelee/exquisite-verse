pub mod poem;
pub use poem::*; 

pub mod ui;
pub use ui::*;

pub mod wasm;   
pub use wasm::*;

use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> Result<(), eframe::Error> {
    ui::run()
}

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("Hello from WASM!")
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

