use wasm_bindgen::prelude::*;
use web_sys::js_sys::Function;

#[wasm_bindgen(start)]
pub fn events() -> Result<(), JsValue> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let say_hello = Closure::wrap(Box::new(move || {
        web_sys::console::log_2(&"Hello World Closures :%s".into(),
                                &"WebAssemblyMan".into());
        document
            .get_element_by_id("counter")
            .expect("should have a button on the page")
            .dyn_ref::<web_sys::HtmlElement>()
            .get(Some("Closures: Hello World"));

    }) as Box);
}

