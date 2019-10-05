extern crate js_sys;
use web_sys::console;
use wasm_bindgen::prelude::*;

pub fn annotate(minefield: Vec<String>) -> Vec<String> {
    minefield
        .iter()
        .map(|row| row.split("").collect::<Vec<&str>>())
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .filter(|c| !c.is_empty())
                .enumerate()
                .map(|(x, cell)| get_annotation(&minefield, y, x, cell))
                .collect::<String>()
        })
        .collect()
}

fn get_annotation(minefield: &Vec<String>, y: usize, x: usize, current_cell: &str) -> char {
    match current_cell {
        " " => match std::char::from_digit(mines_total(minefield, y, x), 10) {
            Some('0') => ' ',
            Some(val) => val,
            None => unreachable!(),
        },
        val => val.chars().next().unwrap(),
    }
}

fn mines_total(matrix: &Vec<String>, y: usize, x: usize) -> u32 {
    (if x != 0 {
        is_cell_mined(matrix, y + 1, x - 1) + is_cell_mined(matrix, y, x - 1)
    } else {
        0
    } + if y != 0 {
        is_cell_mined(matrix, y - 1, x) + is_cell_mined(matrix, y - 1, x + 1)
    } else {
        0
    } + if x != 0 && y != 0 {
        is_cell_mined(matrix, y - 1, x - 1)
    } else {
        0
    } + is_cell_mined(matrix, y, x + 1)
        + is_cell_mined(matrix, y + 1, x + 1)
        + is_cell_mined(matrix, y + 1, x)) as u32
}

fn is_cell_mined(matrix: &Vec<String>, y: usize, x: usize) -> usize {
    match matrix.get(y) {
        Some(row) => row
            .chars()
            .enumerate()
            .filter(|(i, value)| value == &'*' && &x == i)
            .count(),
        None => 0,
    }
}


pub enum LEVEL {
    EASY,
    MEDIUM,
    HARD
}

pub fn generate(level: LEVEL) -> Vec<String> {
    let size = match level {
        LEVEL::EASY =>  10,
        LEVEL::MEDIUM => 50,
        LEVEL::HARD => 100
    };

    let mut mines_nbr : usize = size;
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
        }).collect::<Vec<&str>>().join(" ")
    }).collect();

    // for (i, line) in a.iter().enumerate() {
    //     console::log_1(&JsValue::from(i as u32));
    //     console::log_1(&JsValue::from_str(line));
    //     console::log_1(&JsValue::from_str(line));
    // }
    a
}