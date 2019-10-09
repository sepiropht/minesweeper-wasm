mod board;

use wasm_bindgen::prelude::*;
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

    // Your code goes here!
  //  console::log_1(&JsValue::from_str("Hello world!"));
    let board = board::generate(board::LEVEL::EASY);
    //console::log_1(&JsValue::from(board.len() as u32));
    let boardDom = document.query_selector("#board").expect("not fail").unwrap();
   // console::log_1(&JsValue::from_str("taille"));
    //console::log_1(&JsValue::from(board::annotate(board).len() as u32));
   // console::log_1(&JsValue::from_str("taille"));
    let final_board = board::annotate(board);
    for (y, row) in final_board.iter().enumerate() {
        console::log_1(&JsValue::from_str(&"BEGINNNNNNNNNNNNNNNNNNNNNNNNNNNN row"));
        console::log_1(&JsValue::from(row));
      
     //   console::log_1(&JsValue::from_str(row));
        row.chars().enumerate().for_each(|(x,_)| {
            let div = document.create_element("button").expect("no global `window` exists");
            console::log_1(&JsValue::from_str(&"x begin"));
            console::log_1(&JsValue::from(x as u32));
            console::log_1(&JsValue::from_str(&"x end"));
            div.set_class_name("square");
            div.set_attribute("data-x", &format!("{}", x)).expect("no global `window` exists");
            div.set_attribute("data-y", &format!("{}", y)).expect("no global `window` exists");
            boardDom.append_child(&div).expect("not fail");
        });
                  console::log_1(&JsValue::from_str(&"ENNNNNNNNNNNNNDDDDDDDDDDD! row"));
    }

        //     console::log_1(&JsValue::from(i as u32));
        //     console::log_1(&JsValue::from_str(&line));
        //     console::log_1(&JsValue::from_str(&line));
    //console::log_1(&JsValue::from(&board));
    //console::log_1(&JsValue::from_str(&board::annotate(board)));

    Ok(())
}
