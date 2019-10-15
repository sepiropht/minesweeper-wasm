mod board;

use std::sync::{Arc, Mutex};
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
    init();
    let restart_button = document
        .query_selector("#score-restart-button")
        .expect("dom node")
        .unwrap();

    let closure3 = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        init();
    }) as Box<dyn FnMut(_)>);

    restart_button
        .add_event_listener_with_callback("click", closure3.as_ref().unchecked_ref())
        .expect("error");

    fn init() {
        let board = board::generate(board::LEVEL::EASY);
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let boardDom = document
            .query_selector("#board")
            .expect("not fail")
            .unwrap();
        let final_board = board::annotate(board);
        let width = final_board.len();
        let global_state: Arc<Mutex<Vec<web_sys::Element>>> = Arc::new(Mutex::new(vec![]));

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
                div.set_attribute("id", &format!("cell-{}-{}", x, y))
                    .expect("no global `window` exists");
                div.set_attribute("status", &"")
                    .expect("no global `window` exists");
                div.set_attribute("value", &format!("{}", value))
                    .expect("no global `window` exists");
                let ref mut merrr = *global_state.lock().unwrap();
                merrr.push(div.clone());
                boardDom.append_child(&div).expect("not fail");
                let clone_global = global_state.clone();
                let cloned_div = div.clone();
                let cloned_div1 = div.clone();
                let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                    // let ref mut global_state = *global_state.lock().unwrap();
                    console::log_1(&JsValue::from(x as u32));
                    console::log_1(&JsValue::from(y as u32));
                    console::log_1(&JsValue::from_str(&value.to_string()));
                    if div.get_attribute("status") == Some("empty".to_string()) {
                    } else if value == '*' {
                        div.set_class_name("square bombed trigger");
                        div.set_attribute("status", "bombed")
                            .expect("no global `window` exists");
                        //  web_sys::window().unwrap().document().unwrap().query_selector_all("#board button").expect("nodelist");
                        let ref mut global_state = *clone_global.lock().unwrap();
                        global_state.iter().for_each(|node| {
                            node.set_attribute("disabled", "false");
                        });
                    } else if value.is_digit(10) {
                        div.set_attribute("status", "clicked")
                            .expect("no global `window` exists");
                        div.set_class_name(&format!("square cleared bomb-{}", value));
                        div.set_inner_html(&value.to_string());
                    } else {
                        div.set_class_name("square cleared");
                        board::clear_adjacent_cells(x, y, width);
                    }
                }) as Box<dyn FnMut(_)>);

                let closure2 = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    event.prevent_default();
                    console::log_1(&JsValue::from(x as u32));
                    console::log_1(&JsValue::from(y as u32));
                    console::log_1(&JsValue::from_str(&value.to_string()));
                    let score_node = web_sys::window()
                        .expect("no global `window` exists")
                        .document()
                        .expect("should have a document on window")
                        .query_selector("#score-bomb-count")
                        .expect("dom node")
                        .unwrap();
                    use std::str::FromStr;
                    let mut score = u32::from_str(&score_node.inner_html()).unwrap();
                    if cloned_div1.get_attribute("status") == Some("".to_string()) {
                        cloned_div1
                            .set_attribute("status", "flagged")
                            .expect("no global `window` exists");
                        cloned_div1.set_class_name("square flagged");
                        score -= 1;
                        score_node.set_inner_html(&format!("00{}", score));
                    } else {
                        cloned_div1
                            .set_attribute("status", "")
                            .expect("no global `window` exists");
                        cloned_div1.set_class_name("square");
                        score += 1;
                        score_node.set_inner_html(&format!("00{}", score));
                    }
                }) as Box<dyn FnMut(_)>);

                cloned_div
                    .add_event_listener_with_callback(
                        "contextmenu",
                        closure2.as_ref().unchecked_ref(),
                    )
                    .expect("error");
                closure2.forget();

                cloned_div
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .expect("error");
                closure.forget();
            });
        }
    }

    Ok(())
}
