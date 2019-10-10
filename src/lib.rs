mod board;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let board = board::generate(board::LEVEL::EASY);
    let boardDom = document
        .query_selector("#board")
        .expect("not fail")
        .unwrap();

    let final_board = board::annotate(board);
    for (y, row) in final_board.iter().enumerate() {

        row.chars().enumerate().for_each(|(x, value)| {
            let div = document
                .create_element("button")
                .expect("no global `window` exists");
            div.set_class_name("square");
            div.set_attribute("data-x", &format!("{}", x))
                .expect("no global `window` exists");
            div.set_attribute("data-y", &format!("{}", y))
                .expect("no global `window` exists");
            boardDom.append_child(&div).expect("not fail");
            let cloned_div = div.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                console::log_1(&JsValue::from(x as u32));
                console::log_1(&JsValue::from(y as u32));
                console::log_1(&JsValue::from_str(&value.to_string()));
                if value == '*' {
                    div.set_class_name("square bombed trigger");
                } else if value.is_digit(10) {
                     div.set_class_name(&format!("square cleared bomb-{}", value));
                     div.set_inner_html(&value.to_string());
                } else {
                    div.set_class_name("square cleared");
                }
            }) as Box<dyn FnMut(_)>);

            cloned_div.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("error");
            closure.forget();
        });
    }

    Ok(())
}
