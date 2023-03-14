use std::convert::TryInto;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn do_stuff() -> Result<(), JsValue> {
    log("Hello from Rust!");

    return Ok(());
}
