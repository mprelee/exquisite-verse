use wasm_bindgen::prelude::*;
use crate::ui::ExquisiteVerse;

// This function is called automatically by Trunk (no JS glue needed!)
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    // Better panic messages in the browser console
    console_error_panic_hook::set_once();

    // Set up the canvas and start the app
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("the_canvas_id")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    eframe::web::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|_cc| Ok(Box::new(ExquisiteVerse::new()))),
        )
        .await
}
