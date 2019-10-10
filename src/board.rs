extern crate js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(field: Vec<String>) -> Vec<String> {
    let height = field.len() as i32;
    (0..height)
        .map(|y| {
            let width = field[y as usize].len() as i32;
            (0..width)
                .map(|x| {
                    if field[y as usize].as_bytes()[x as usize] == b'*' {
                        '*'
                    } else {
                        match NEIGBOURHOOD_OFFSETS
                            .iter()
                            .map(|&(ox, oy)| (x + ox, y + oy))
                            .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                            .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + '0' as u8) as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}

pub enum LEVEL {
    EASY,
    MEDIUM,
    HARD,
}

pub fn generate(level: LEVEL) -> Vec<String> {
    let size = match level {
        LEVEL::EASY => 9,
        LEVEL::MEDIUM => 48,
        LEVEL::HARD => 98,
    };

    let mut mines_nbr: usize = size + 1;
    let mut indices: Vec<usize> = (0..mines_nbr * mines_nbr).map(|indice| indice).collect();
    let mut mine_locations = vec![];

    for _ in 0..mines_nbr {
        let index = js_sys::Math::floor(js_sys::Math::random() * indices.len() as f64) as usize;
        mine_locations.push(index);
        indices.remove(index);
        mines_nbr -= 1;
    }
    mine_locations.sort();

    let mut num = 0;
    (0..size)
        .enumerate()
        .map(|_| {
            (0..size)
                .map(|_| {
                    if mine_locations.iter().any(|index| index == &num) {
                        num += 1;
                        "*"
                    } else {
                        num += 1;
                        " "
                    }
                })
                .collect()
        })
        .collect()
}

pub fn clear_adjacent_cells(x: usize, y: usize, len: usize) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let current_square = document
        .query_selector(&format!("#cell-{}-{}", x, y))
        .expect("dom node")
        .unwrap();
    if x < len + 1 && y < len + 1 {
        if current_square.get_attribute("status") == Some("empty".to_string()) {
            NEIGBOURHOOD_OFFSETS
                .iter()
                .map(|&(ox, oy)| (x as i32 + ox as i32, y as i32 + oy as i32))
                .filter(|&(x, y)| {
                    (0 <= x && x < len as i32) && (0 <= y && y < len as i32)
                })
                .for_each(|_| clear_adjacent_cells(x, y, len));
        } else {
            let value = current_square.get_attribute("value").unwrap();
            if value.chars().next().unwrap().is_numeric() {
                current_square.set_class_name(&format!("cleared bomb-{}", value));
            } else {
                current_square.set_class_name(&"square cleared");
            }
        }
    }
}
