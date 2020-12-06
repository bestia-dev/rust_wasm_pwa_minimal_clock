use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use web_sys::console;

const VEC_DAY_NAME: [&'static str; 7] = [
    "sunday",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
];

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    debug_write(&format!(
        "rust_wasm_pwa_minimal_clock v{}",
        env!("CARGO_PKG_VERSION")
    ));
    unwrap!(window().resize_to(400, 200));

    write_time_to_screen();
    // every 1s write time to screen
    set_interval(Box::new(write_time_to_screen), 1000);

    Ok(())
}

/// write local time to screen
pub fn write_time_to_screen() {
    // local time and date
    use js_sys::*;
    let now = Date::new_0();
    let now_time = format!(
        "{:02}:{:02}:{:02}",
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds(),
    );
    let now_date = format!(
        "{:02}.{:02}.{:04}  {}",
        now.get_date(),
        now.get_month() + 1,
        now.get_full_year(),
        VEC_DAY_NAME[now.get_day() as usize],
    );
    let html = format!("<h1>{}</h1><p>{}</p>", now_time, now_date);

    let div_for_wasm_html_injecting = get_element_by_id("div_for_wasm_html_injecting");
    div_for_wasm_html_injecting.set_inner_html(&html);
}

/// A high-level wrapper for web_sys::window.set_interval_with_callback_and_timeout_and_arguments_0:
pub fn set_interval(handler: Box<dyn Fn()>, timeout: i32) {
    let callback = Closure::wrap(handler as Box<dyn Fn()>);
    use wasm_bindgen::JsCast;
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("Problem setting interval");
    callback.forget();
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
