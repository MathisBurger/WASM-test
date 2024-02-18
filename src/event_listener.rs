use wasm_bindgen::prelude::*;
use web_sys::js_sys::Function;

#[wasm_bindgen(start)]
pub fn events() -> Result<(), JsValue> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let func = Function::new_no_args("");
    document.add_event_listener_with_callback("click", &func)?;
    Ok(())
}