use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    debug_write(&format!("wasm app version: {}", env!("CARGO_PKG_VERSION")));

    let div_for_wasm_html_injecting = get_element_by_id("div_for_wasm_html_injecting");
    write_time_to_screen(&div_for_wasm_html_injecting);

    Ok(())
}

/// write local time to screen
pub fn write_time_to_screen(div_for_wasm_html_injecting: &web_sys::Element) {
    // local time and date
    use js_sys::*;
    let now = Date::new_0();
    let now_time = format!(
        "{}:{}:{}",
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds()
    );
    let now_date = format!(
        "{}.{}.{}",
        now.get_date(),
        now.get_month() + 1,
        now.get_full_year()
    );
    let html = format!("<h2>{}</h2><p>{}</p>", now_time, now_date);

    div_for_wasm_html_injecting.set_inner_html(&html);
}

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}
