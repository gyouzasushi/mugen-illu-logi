mod gen;
use gen::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen(h: usize, w: usize) -> Vec<i32> {
    let ret = gen_inner(h, w);
    ret.0.iter().map(|&f| f as i32).collect()
}
