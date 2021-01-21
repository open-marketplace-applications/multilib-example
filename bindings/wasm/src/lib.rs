use wasm_bindgen::prelude::*;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

/// Convert errors so they are readable in JS
pub fn js_err<T: ToString>(error: T) -> JsValue {
    error.to_string().into()
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Greet function from our Rust library
#[wasm_bindgen]
pub fn greet() -> String {
    multilib_example::greet()
}