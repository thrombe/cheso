pub mod run;

use wasm_bindgen::prelude::wasm_bindgen;

#[allow(dead_code)]
fn main() {
    run::run().unwrap();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    run::run().unwrap();
}
