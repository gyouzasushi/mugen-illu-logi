mod gen;
mod vis;
extern crate console_error_panic_hook;
use gen::*;
use itertools::Itertools;
use rand::prelude::*;
use std::panic;
use vis::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen(h: usize, w: usize, seed: u64) -> Vec<i32> {
    let ret = gen_inner(h, w, seed);
    ret.0.iter().map(|&f| f as i32).collect()
}

#[wasm_bindgen]
pub fn gen_seed() -> String {
    rand::thread_rng().gen_range(0..1_u64 << 53).to_string()
}

#[wasm_bindgen]
pub fn vis_grid(h: usize, w: usize, d: i32, board: Vec<i32>) -> String {
    let board = parse_board(h, w, &board);
    vis_grid_inner(h, w, d, &board)
}

#[wasm_bindgen]
pub fn vis_gif(h: usize, w: usize, d: u16, boards: Vec<i32>, max_turn: usize) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    assert_eq!(max_turn * h * w, boards.len());
    let boards = (0..max_turn)
        .map(|t| parse_board_unwrap(h, w, &boards[t * h * w..(t + 1) * h * w]))
        .collect_vec();
    vis_gif_inner(h, w, d, &boards)
}

#[wasm_bindgen]
pub fn vis_board(
    h: usize,
    w: usize,
    board: Vec<i32>,
    hints: Vec<i32>,
    offset_y: i32,
    offset_x: i32,
) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let board = parse_board(h, w, &board);
    let hints = parse_hints(h, w, &hints);
    vis_board_inner(h, w, &board, &hints, "black", offset_y, offset_x)
}

#[wasm_bindgen]
pub fn vis_gaming_boards(
    h: usize,
    w: usize,
    board: Vec<i32>,
    hints: Vec<i32>,
    offset_y: i32,
    offset_x: i32,
) -> String {
    let board = parse_board(h, w, &board);
    let hints = parse_hints(h, w, &hints);
    vis_gaming_boards_inner(h, w, &board, &hints, offset_y, offset_x)
}

#[wasm_bindgen]
pub fn set(
    y: usize,
    x: usize,
    val: Option<bool>,
    h: usize,
    w: usize,
    board: Vec<i32>,
    hints: Vec<i32>,
) -> Vec<i32> {
    let mut board = parse_board(h, w, &board);
    let hints = parse_hints(h, w, &hints);
    set_inner(y, x, val, &mut board, &hints);
    board
        .iter()
        .flatten()
        .map(|x| match x {
            Some(false) => 0,
            Some(true) => 1,
            None => 2,
        })
        .collect::<Vec<_>>()
}

#[wasm_bindgen]
pub fn vis_cursor(h: usize, w: usize, y: usize, x: usize, offset_y: i32, offset_x: i32) -> String {
    vis_cursor_inner(h, w, y, x, offset_y, offset_x)
}

fn parse_board(h: usize, w: usize, board: &[i32]) -> Vec<Vec<Option<bool>>> {
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| match board[y * w + x] {
                    0 => Some(false),
                    1 => Some(true),
                    2 => None,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
fn parse_board_unwrap(h: usize, w: usize, board: &[i32]) -> Vec<Vec<bool>> {
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| match board[y * w + x] {
                    0 | 2 => false,
                    1 => true,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
fn parse_hints(h: usize, w: usize, hints: &Vec<i32>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut ret: Vec<Vec<i32>> = vec![];
    let mut i = 0;
    while i < hints.len() {
        let k = hints[i] as usize;
        i += 1;
        ret.push(hints[i..(i + k)].to_vec());
        i += k;
    }
    assert_eq!(ret.len(), h + w);
    (ret[..h].to_vec(), ret[h..].to_vec())
}
