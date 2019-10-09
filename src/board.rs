extern crate js_sys;
use web_sys::console;
use wasm_bindgen::prelude::*;

static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(field: Vec<String>) -> Vec<String> {
    let height = field.len() as i32;
    (0..height).map(|y| {
        let width = field[y as usize].len() as i32;
        (0..width).map(|x| {
            if field[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                match NEIGBOURHOOD_OFFSETS.iter()
                    .map(|&(ox, oy)| (x + ox, y + oy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
                    .count() {
                        0 => ' ',
                        n => (n as u8 + '0' as u8) as char
                    }
            }
        }).collect()
    }).collect()
}


pub enum LEVEL {
    EASY,
    MEDIUM,
    HARD
}

pub fn generate(level: LEVEL) -> Vec<String> {
    let size = match level {
        LEVEL::EASY =>  9,
        LEVEL::MEDIUM => 48,
        LEVEL::HARD => 98
    };

    let mut mines_nbr : usize = size + 1;
    let mut indices : Vec<usize> = (0..mines_nbr * mines_nbr).map(|indice| indice).collect();
    let mut mine_locations = vec![];


    for _ in 0..mines_nbr {
         let index = js_sys::Math::floor(js_sys::Math::random() * indices.len() as f64) as usize;
         mine_locations.push(index);
         indices.remove(index);
         mines_nbr -= 1;
    }
    mine_locations.sort();
    // console::log_1(&JsValue::from_str(&"in generated function"));
    // console::log_1(&JsValue::from(mine_locations.len() as u32));
    // console::log_1(&JsValue::from_str(&"in generated function"));

    
    let mut num = 0;
    let a: Vec<String> = (0..size).enumerate().map(|_| {
        (0..size).map(|_| {
            if mine_locations.iter().any(|index| index == &num) {
                console::log_1(&JsValue::from_str("true"));
                num += 1;
                "*"
            } else {
                console::log_1(&JsValue::from_str("false"));
                num += 1;
                " "
            }
        }).collect()
    }).collect();

    for (i, line) in a.iter().enumerate() {
        console::log_1(&JsValue::from(line.len() as u32));
        console::log_1(&JsValue::from_str(&"line.length"));
        console::log_1(&JsValue::from_str(line));
    }
    a
}