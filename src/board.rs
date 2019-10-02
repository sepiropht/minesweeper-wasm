//extern crate js_sys;

pub fn annotate(minefield: &[&str]) -> String {
    minefield
        .iter()
        .map(|row| row.split("").collect::<Vec<&str>>())
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .filter(|c| !c.is_empty())
                .enumerate()
                .map(|(x, cell)| get_annotation(minefield, y, x, cell))
                .collect::<String>()
        })
        .collect()
}

fn get_annotation(minefield: &[&str], y: usize, x: usize, current_cell: &str) -> char {
    match current_cell {
        " " => match std::char::from_digit(mines_total(minefield, y, x), 10) {
            Some('0') => ' ',
            Some(val) => val,
            None => unreachable!(),
        },
        val => val.chars().next().unwrap(),
    }
}

fn mines_total(matrix: &[&str], y: usize, x: usize) -> u32 {
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

fn is_cell_mined(matrix: &[&str], y: usize, x: usize) -> usize {
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

pub fn generate(level: LEVEL) -> String {
    let (width, heigth) = match level {
        EASY => (10, 10),
        MEDIUM => (50, 50),
        HARD => (100, 100)
    };

    unimplemented!();
}