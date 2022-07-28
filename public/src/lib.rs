extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;

mod error_log;

use std::f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

static TRI_TABLE: [[i32; 16]; 256] = [
    [-1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  8,  3, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 1,  9,  0, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  1,  9,  8,  3,  1, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 2, 10,  1, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  8,  3,  1,  2, 10, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 9,  2, 10,  9,  0,  2, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 3,  2, 10,  3, 10,  8,  8, 10,  9, -1,  0,  0,  0,  0,  0,  0],
    [ 2,  3, 11, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [11,  0,  8, 11,  2,  0, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 1,  9,  0,  2,  3, 11, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 2,  1,  9,  2,  9, 11, 11,  9,  8, -1,  0,  0,  0,  0,  0,  0],
    [ 3, 10,  1,  3, 11, 10, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 1,  0,  8,  1,  8, 10, 10,  8, 11, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  3, 11,  0, 11,  9,  9, 11, 10, -1,  0,  0,  0,  0,  0,  0],
    [11, 10,  9, 11,  9,  8, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  7,  8, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  3,  0,  4,  7,  3, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  7,  8,  9,  0,  1, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 9,  4,  7,  9,  7,  1,  1,  7,  3, -1,  0,  0,  0,  0,  0,  0],
    [ 4,  7,  8,  1,  2, 10, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  3,  0,  4,  7,  3,  2, 10,  1, -1,  0,  0,  0,  0,  0,  0],
    [ 2,  9,  0,  2, 10,  9,  4,  7,  8, -1,  0,  0,  0,  0,  0,  0],
    [ 3,  2,  7,  7,  9,  4,  7,  2,  9,  9,  2, 10, -1,  0,  0,  0],
    [ 8,  4,  7,  3, 11,  2, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 7, 11,  2,  7,  2,  4,  4,  2,  0, -1,  0,  0,  0,  0,  0,  0],
    [ 2,  3, 11,  1,  9,  0,  8,  4,  7, -1,  0,  0,  0,  0,  0,  0],
    [ 2,  1,  9,  2,  9,  4,  2,  4, 11, 11,  4,  7, -1,  0,  0,  0],
    [10,  3, 11, 10,  1,  3,  8,  4,  7, -1,  0,  0,  0,  0,  0,  0],
    [ 4,  7,  0,  0, 10,  1,  7, 10,  0,  7, 11, 10, -1,  0,  0,  0],
    [ 8,  4,  7,  0,  3, 11,  0, 11,  9,  9, 11, 10, -1,  0,  0,  0],
    [ 7,  9,  4,  7, 11,  9,  9, 11, 10, -1,  0,  0,  0,  0,  0,  0],
    [ 4,  9,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  3,  0,  4,  9,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  5,  4,  0,  1,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  8,  3,  4,  3,  5,  5,  3,  1, -1,  0,  0,  0,  0,  0,  0],
    [ 1,  2, 10,  9,  5,  4, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  9,  5,  8,  3,  0,  1,  2, 10, -1,  0,  0,  0,  0,  0,  0],
    [10,  5,  4, 10,  4,  2,  2,  4,  0, -1,  0,  0,  0,  0,  0,  0],
    [ 4,  8,  3,  4,  3,  2,  4,  2,  5,  5,  2, 10, -1,  0,  0,  0],
    [ 2,  3, 11,  5,  4,  9, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [11,  0,  8, 11,  2,  0,  9,  5,  4, -1,  0,  0,  0,  0,  0,  0],
    [ 5,  0,  1,  5,  4,  0,  3, 11,  2, -1,  0,  0,  0,  0,  0,  0],
    [11,  2,  8,  8,  5,  4,  2,  5,  8,  2,  1,  5, -1,  0,  0,  0],
    [ 3, 10,  1,  3, 11, 10,  5,  4,  9, -1,  0,  0,  0,  0,  0,  0],
    [ 9,  5,  4,  1,  0,  8,  1,  8, 10, 10,  8, 11, -1,  0,  0,  0],
    [10,  5, 11, 11,  0,  3, 11,  5,  0,  0,  5,  4, -1,  0,  0,  0],
    [ 4, 10,  5,  4,  8, 10, 10,  8, 11, -1,  0,  0,  0,  0,  0,  0],
    [ 7,  9,  5,  7,  8,  9, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  9,  5,  0,  5,  3,  3,  5,  7, -1,  0,  0,  0,  0,  0,  0],
    [ 8,  0,  1,  8,  1,  7,  7,  1,  5, -1,  0,  0,  0,  0,  0,  0],
    [ 3,  1,  5,  3,  5,  7, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 7,  9,  5,  7,  8,  9,  1,  2, 10, -1,  0,  0,  0,  0,  0,  0],
    [ 1,  2, 10,  0,  9,  5,  0,  5,  3,  3,  5,  7, -1,  0,  0,  0],
    [ 7,  8,  5,  5,  2, 10,  8,  2,  5,  8,  0,  2, -1,  0,  0,  0],
    [10,  3,  2, 10,  5,  3,  3,  5,  7, -1,  0,  0,  0,  0,  0,  0],
    [ 9,  7,  8,  9,  5,  7, 11,  2,  3, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  9,  2,  2,  7, 11,  2,  9,  7,  7,  9,  5, -1,  0,  0,  0],
    [ 3, 11,  2,  8,  0,  1,  8,  1,  7,  7,  1,  5, -1,  0,  0,  0],
    [ 2,  7, 11,  2,  1,  7,  7,  1,  5, -1,  0,  0,  0,  0,  0,  0],
    [11,  1,  3, 11, 10,  1,  7,  8,  9,  7,  9,  5, -1,  0,  0,  0],
    [11, 10,  1, 11,  1,  7,  7,  1,  0,  7,  0,  9,  7,  9,  5, -1],
    [ 5,  7,  8,  5,  8, 10, 10,  8,  0, 10,  0,  3, 10,  3, 11, -1],
    [11, 10,  5, 11,  5,  7, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [10,  6,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  8,  3, 10,  6,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 9,  0,  1,  5, 10,  6, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  1,  9,  8,  3,  1, 10,  6,  5, -1,  0,  0,  0,  0,  0,  0],
    [ 6,  1,  2,  6,  5,  1, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 6,  1,  2,  6,  5,  1,  0,  8,  3, -1,  0,  0,  0,  0,  0,  0],
    [ 5,  9,  0,  5,  0,  6,  6,  0,  2, -1,  0,  0,  0,  0,  0,  0],
    [ 6,  5,  2,  2,  8,  3,  5,  8,  2,  5,  9,  8, -1,  0,  0,  0],
    [ 2,  3, 11, 10,  6,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0, 11,  2,  0,  8, 11,  6,  5, 10, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  1,  9,  3, 11,  2, 10,  6,  5, -1,  0,  0,  0,  0,  0,  0],
    [10,  6,  5,  2,  1,  9,  2,  9, 11, 11,  9,  8, -1,  0,  0,  0],
    [11,  6,  5, 11,  5,  3,  3,  5,  1, -1,  0,  0,  0,  0,  0,  0],
    [11,  6,  8,  8,  1,  0,  8,  6,  1,  1,  6,  5, -1,  0,  0,  0],
    [ 0,  3, 11,  0, 11,  6,  0,  6,  9,  9,  6,  5, -1,  0,  0,  0],
    [ 5, 11,  6,  5,  9, 11, 11,  9,  8, -1,  0,  0,  0,  0,  0,  0],
    [ 7,  8,  4,  6,  5, 10, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 3,  4,  7,  3,  0,  4,  5, 10,  6, -1,  0,  0,  0,  0,  0,  0],
    [ 6,  5, 10,  7,  8,  4,  9,  0,  1, -1,  0,  0,  0,  0,  0,  0],
    [ 5, 10,  6,  9,  4,  7,  9,  7,  1,  1,  7,  3, -1,  0,  0,  0],
    [ 1,  6,  5,  1,  2,  6,  7,  8,  4, -1,  0,  0,  0,  0,  0,  0],
    [ 7,  0,  4,  7,  3,  0,  6,  5,  1,  6,  1,  2, -1,  0,  0,  0],
    [ 4,  7,  8,  5,  9,  0,  5,  0,  6,  6,  0,  2, -1,  0,  0,  0],
    [ 2,  6,  5,  2,  5,  3,  3,  5,  9,  3,  9,  4,  3,  4,  7, -1],
    [ 4,  7,  8,  5, 10,  6, 11,  2,  3, -1,  0,  0,  0,  0,  0,  0],
    [ 6,  5, 10,  7, 11,  2,  7,  2,  4,  4,  2,  0, -1,  0,  0,  0],
    [ 4,  7,  8,  9,  0,  1,  6,  5, 10,  3, 11,  2, -1,  0,  0,  0],
    [ 6,  5, 10, 11,  4,  7, 11,  2,  4,  4,  2,  9,  9,  2,  1, -1],
    [ 7,  8,  4, 11,  6,  5, 11,  5,  3,  3,  5,  1, -1,  0,  0,  0],
    [ 0,  4,  7,  0,  7,  1,  1,  7, 11,  1, 11,  6,  1,  6,  5, -1],
    [ 4,  7,  8,  9,  6,  5,  9,  0,  6,  6,  0, 11, 11,  0,  3, -1],
    [ 7, 11,  4, 11,  9,  4, 11,  5,  9, 11,  6,  5, -1,  0,  0,  0],
    [10,  4,  9, 10,  6,  4, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [10,  4,  9, 10,  6,  4,  8,  3,  0, -1,  0,  0,  0,  0,  0,  0],
    [ 1, 10,  6,  1,  6,  0,  0,  6,  4, -1,  0,  0,  0,  0,  0,  0],
    [ 4,  8,  6,  6,  1, 10,  6,  8,  1,  1,  8,  3, -1,  0,  0,  0],
    [ 9,  1,  2,  9,  2,  4,  4,  2,  6, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  8,  3,  9,  1,  2,  9,  2,  4,  4,  2,  6, -1,  0,  0,  0],
    [ 0,  2,  6,  0,  6,  4, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 3,  4,  8,  3,  2,  4,  4,  2,  6, -1,  0,  0,  0,  0,  0,  0],
    [ 4, 10,  6,  4,  9, 10,  2,  3, 11, -1,  0,  0,  0,  0,  0,  0],
    [ 8,  2,  0,  8, 11,  2,  4,  9, 10,  4, 10,  6, -1,  0,  0,  0],
    [ 2,  3, 11,  1, 10,  6,  1,  6,  0,  0,  6,  4, -1,  0,  0,  0],
    [ 8, 11,  2,  8,  2,  4,  4,  2,  1,  4,  1, 10,  4, 10,  6, -1],
    [ 3, 11,  1,  1,  4,  9, 11,  4,  1, 11,  6,  4, -1,  0,  0,  0],
    [ 6,  4,  9,  6,  9, 11, 11,  9,  1, 11,  1,  0, 11,  0,  8, -1],
    [11,  0,  3, 11,  6,  0,  0,  6,  4, -1,  0,  0,  0,  0,  0,  0],
    [ 8, 11,  6,  8,  6,  4, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 6,  7,  8,  6,  8, 10, 10,  8,  9, -1,  0,  0,  0,  0,  0,  0],
    [ 3,  0,  7,  7, 10,  6,  0, 10,  7,  0,  9, 10, -1,  0,  0,  0],
    [ 1, 10,  6,  1,  6,  7,  1,  7,  0,  0,  7,  8, -1,  0,  0,  0],
    [ 6,  1, 10,  6,  7,  1,  1,  7,  3, -1,  0,  0,  0,  0,  0,  0],
    [ 9,  1,  8,  8,  6,  7,  8,  1,  6,  6,  1,  2, -1,  0,  0,  0],
    [ 7,  3,  0,  7,  0,  6,  6,  0,  9,  6,  9,  1,  6,  1,  2, -1],
    [ 8,  6,  7,  8,  0,  6,  6,  0,  2, -1,  0,  0,  0,  0,  0,  0],
    [ 2,  6,  7,  2,  7,  3, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [11,  2,  3,  6,  7,  8,  6,  8, 10, 10,  8,  9, -1,  0,  0,  0],
    [ 9, 10,  6,  9,  6,  0,  0,  6,  7,  0,  7, 11,  0, 11,  2, -1],
    [ 3, 11,  2,  0,  7,  8,  0,  1,  7,  7,  1,  6,  6,  1, 10, -1],
    [ 6,  7, 10,  7,  1, 10,  7,  2,  1,  7, 11,  2, -1,  0,  0,  0],
    [ 1,  3, 11,  1, 11,  9,  9, 11,  6,  9,  6,  7,  9,  7,  8, -1],
    [ 6,  7, 11,  9,  1,  0, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  0,  7,  0,  6,  7,  0, 11,  6,  0,  3, 11, -1,  0,  0,  0],
    [ 6,  7, 11, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 6, 11,  7, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 3,  0,  8, 11,  7,  6, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 6, 11,  7,  9,  0,  1, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 1,  8,  3,  1,  9,  8,  7,  6, 11, -1,  0,  0,  0,  0,  0,  0],
    [11,  7,  6,  2, 10,  1, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 1,  2, 10,  0,  8,  3, 11,  7,  6, -1,  0,  0,  0,  0,  0,  0],
    [ 9,  2, 10,  9,  0,  2, 11,  7,  6, -1,  0,  0,  0,  0,  0,  0],
    [11,  7,  6,  3,  2, 10,  3, 10,  8,  8, 10,  9, -1,  0,  0,  0],
    [ 2,  7,  6,  2,  3,  7, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  7,  6,  8,  6,  0,  0,  6,  2, -1,  0,  0,  0,  0,  0,  0],
    [ 7,  2,  3,  7,  6,  2,  1,  9,  0, -1,  0,  0,  0,  0,  0,  0],
    [ 8,  7,  9,  9,  2,  1,  9,  7,  2,  2,  7,  6, -1,  0,  0,  0],
    [ 6, 10,  1,  6,  1,  7,  7,  1,  3, -1,  0,  0,  0,  0,  0,  0],
    [ 6, 10,  1,  6,  1,  0,  6,  0,  7,  7,  0,  8, -1,  0,  0,  0],
    [ 7,  6,  3,  3,  9,  0,  6,  9,  3,  6, 10,  9, -1,  0,  0,  0],
    [ 6,  8,  7,  6, 10,  8,  8, 10,  9, -1,  0,  0,  0,  0,  0,  0],
    [ 8,  6, 11,  8,  4,  6, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [11,  3,  0, 11,  0,  6,  6,  0,  4, -1,  0,  0,  0,  0,  0,  0],
    [ 6,  8,  4,  6, 11,  8,  0,  1,  9, -1,  0,  0,  0,  0,  0,  0],
    [ 1,  9,  3,  3,  6, 11,  9,  6,  3,  9,  4,  6, -1,  0,  0,  0],
    [ 8,  6, 11,  8,  4,  6, 10,  1,  2, -1,  0,  0,  0,  0,  0,  0],
    [ 2, 10,  1, 11,  3,  0, 11,  0,  6,  6,  0,  4, -1,  0,  0,  0],
    [11,  4,  6, 11,  8,  4,  2, 10,  9,  2,  9,  0, -1,  0,  0,  0],
    [ 4,  6, 11,  4, 11,  9,  9, 11,  3,  9,  3,  2,  9,  2, 10, -1],
    [ 3,  8,  4,  3,  4,  2,  2,  4,  6, -1,  0,  0,  0,  0,  0,  0],
    [ 2,  0,  4,  2,  4,  6, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  1,  9,  3,  8,  4,  3,  4,  2,  2,  4,  6, -1,  0,  0,  0],
    [ 9,  2,  1,  9,  4,  2,  2,  4,  6, -1,  0,  0,  0,  0,  0,  0],
    [ 6, 10,  4,  4,  3,  8,  4, 10,  3,  3, 10,  1, -1,  0,  0,  0],
    [ 1,  6, 10,  1,  0,  6,  6,  0,  4, -1,  0,  0,  0,  0,  0,  0],
    [10,  9,  0, 10,  0,  6,  6,  0,  3,  6,  3,  8,  6,  8,  4, -1],
    [10,  9,  4, 10,  4,  6, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 6, 11,  7,  5,  4,  9, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  8,  3,  9,  5,  4,  7,  6, 11, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  5,  4,  0,  1,  5,  6, 11,  7, -1,  0,  0,  0,  0,  0,  0],
    [ 7,  6, 11,  4,  8,  3,  4,  3,  5,  5,  3,  1, -1,  0,  0,  0],
    [ 2, 10,  1, 11,  7,  6,  5,  4,  9, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  8,  3,  1,  2, 10,  4,  9,  5, 11,  7,  6, -1,  0,  0,  0],
    [ 6, 11,  7, 10,  5,  4, 10,  4,  2,  2,  4,  0, -1,  0,  0,  0],
    [ 6, 11,  7,  5,  2, 10,  5,  4,  2,  2,  4,  3,  3,  4,  8, -1],
    [ 2,  7,  6,  2,  3,  7,  4,  9,  5, -1,  0,  0,  0,  0,  0,  0],
    [ 4,  9,  5,  8,  7,  6,  8,  6,  0,  0,  6,  2, -1,  0,  0,  0],
    [ 3,  6,  2,  3,  7,  6,  0,  1,  5,  0,  5,  4, -1,  0,  0,  0],
    [ 1,  5,  4,  1,  4,  2,  2,  4,  8,  2,  8,  7,  2,  7,  6, -1],
    [ 5,  4,  9,  6, 10,  1,  6,  1,  7,  7,  1,  3, -1,  0,  0,  0],
    [ 4,  9,  5,  7,  0,  8,  7,  6,  0,  0,  6,  1,  1,  6, 10, -1],
    [ 3,  7,  6,  3,  6,  0,  0,  6, 10,  0, 10,  5,  0,  5,  4, -1],
    [ 4,  8,  5,  8, 10,  5,  8,  6, 10,  8,  7,  6, -1,  0,  0,  0],
    [ 5,  6, 11,  5, 11,  9,  9, 11,  8, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  9,  5,  0,  5,  6,  0,  6,  3,  3,  6, 11, -1,  0,  0,  0],
    [ 8,  0, 11, 11,  5,  6, 11,  0,  5,  5,  0,  1, -1,  0,  0,  0],
    [11,  5,  6, 11,  3,  5,  5,  3,  1, -1,  0,  0,  0,  0,  0,  0],
    [10,  1,  2,  5,  6, 11,  5, 11,  9,  9, 11,  8, -1,  0,  0,  0],
    [ 2, 10,  1,  3,  6, 11,  3,  0,  6,  6,  0,  5,  5,  0,  9, -1],
    [ 0,  2, 10,  0, 10,  8,  8, 10,  5,  8,  5,  6,  8,  6, 11, -1],
    [11,  3,  6,  3,  5,  6,  3, 10,  5,  3,  2, 10, -1,  0,  0,  0],
    [ 2,  3,  6,  6,  9,  5,  3,  9,  6,  3,  8,  9, -1,  0,  0,  0],
    [ 5,  0,  9,  5,  6,  0,  0,  6,  2, -1,  0,  0,  0,  0,  0,  0],
    [ 6,  2,  3,  6,  3,  5,  5,  3,  8,  5,  8,  0,  5,  0,  1, -1],
    [ 6,  2,  1,  6,  1,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  9,  5,  8,  5,  3,  3,  5,  6,  3,  6, 10,  3, 10,  1, -1],
    [ 1,  0, 10,  0,  6, 10,  0,  5,  6,  0,  9,  5, -1,  0,  0,  0],
    [ 0,  3,  8, 10,  5,  6, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [10,  5,  6, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [11,  5, 10, 11,  7,  5, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 5, 11,  7,  5, 10, 11,  3,  0,  8, -1,  0,  0,  0,  0,  0,  0],
    [11,  5, 10, 11,  7,  5,  9,  0,  1, -1,  0,  0,  0,  0,  0,  0],
    [ 9,  3,  1,  9,  8,  3,  5, 10, 11,  5, 11,  7, -1,  0,  0,  0],
    [ 2, 11,  7,  2,  7,  1,  1,  7,  5, -1,  0,  0,  0,  0,  0,  0],
    [ 3,  0,  8,  2, 11,  7,  2,  7,  1,  1,  7,  5, -1,  0,  0,  0],
    [ 2, 11,  0,  0,  5,  9,  0, 11,  5,  5, 11,  7, -1,  0,  0,  0],
    [ 9,  8,  3,  9,  3,  5,  5,  3,  2,  5,  2, 11,  5, 11,  7, -1],
    [10,  2,  3, 10,  3,  5,  5,  3,  7, -1,  0,  0,  0,  0,  0,  0],
    [ 5, 10,  7,  7,  0,  8, 10,  0,  7, 10,  2,  0, -1,  0,  0,  0],
    [ 1,  9,  0, 10,  2,  3, 10,  3,  5,  5,  3,  7, -1,  0,  0,  0],
    [ 7,  5, 10,  7, 10,  8,  8, 10,  2,  8,  2,  1,  8,  1,  9, -1],
    [ 7,  5,  1,  7,  1,  3, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  1,  0,  8,  7,  1,  1,  7,  5, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  5,  9,  0,  3,  5,  5,  3,  7, -1,  0,  0,  0,  0,  0,  0],
    [ 7,  5,  9,  7,  9,  8, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  5, 10,  4, 10,  8,  8, 10, 11, -1,  0,  0,  0,  0,  0,  0],
    [11,  3, 10, 10,  4,  5, 10,  3,  4,  4,  3,  0, -1,  0,  0,  0],
    [ 9,  0,  1,  4,  5, 10,  4, 10,  8,  8, 10, 11, -1,  0,  0,  0],
    [ 3,  1,  9,  3,  9, 11, 11,  9,  4, 11,  4,  5, 11,  5, 10, -1],
    [ 8,  4, 11, 11,  1,  2,  4,  1, 11,  4,  5,  1, -1,  0,  0,  0],
    [ 5,  1,  2,  5,  2,  4,  4,  2, 11,  4, 11,  3,  4,  3,  0, -1],
    [11,  8,  4, 11,  4,  2,  2,  4,  5,  2,  5,  9,  2,  9,  0, -1],
    [ 2, 11,  3,  5,  9,  4, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  5, 10,  4, 10,  2,  4,  2,  8,  8,  2,  3, -1,  0,  0,  0],
    [10,  4,  5, 10,  2,  4,  4,  2,  0, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  1,  9,  8,  2,  3,  8,  4,  2,  2,  4, 10, 10,  4,  5, -1],
    [10,  2,  5,  2,  4,  5,  2,  9,  4,  2,  1,  9, -1,  0,  0,  0],
    [ 4,  3,  8,  4,  5,  3,  3,  5,  1, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  4,  5,  0,  5,  1, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  3,  9,  3,  5,  9,  3,  4,  5,  3,  8,  4, -1,  0,  0,  0],
    [ 4,  5,  9, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 7,  4,  9,  7,  9, 11, 11,  9, 10, -1,  0,  0,  0,  0,  0,  0],
    [ 8,  3,  0,  7,  4,  9,  7,  9, 11, 11,  9, 10, -1,  0,  0,  0],
    [ 0,  1,  4,  4, 11,  7,  1, 11,  4,  1, 10, 11, -1,  0,  0,  0],
    [10, 11,  7, 10,  7,  1,  1,  7,  4,  1,  4,  8,  1,  8,  3, -1],
    [ 2, 11,  7,  2,  7,  4,  2,  4,  1,  1,  4,  9, -1,  0,  0,  0],
    [ 0,  8,  3,  1,  4,  9,  1,  2,  4,  4,  2,  7,  7,  2, 11, -1],
    [ 7,  2, 11,  7,  4,  2,  2,  4,  0, -1,  0,  0,  0,  0,  0,  0],
    [ 7,  4, 11,  4,  2, 11,  4,  3,  2,  4,  8,  3, -1,  0,  0,  0],
    [ 7,  4,  3,  3, 10,  2,  3,  4, 10, 10,  4,  9, -1,  0,  0,  0],
    [ 2,  0,  8,  2,  8, 10, 10,  8,  7, 10,  7,  4, 10,  4,  9, -1],
    [ 4,  0,  1,  4,  1,  7,  7,  1, 10,  7, 10,  2,  7,  2,  3, -1],
    [ 4,  8,  7,  1, 10,  2, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 9,  7,  4,  9,  1,  7,  7,  1,  3, -1,  0,  0,  0,  0,  0,  0],
    [ 8,  7,  0,  7,  1,  0,  7,  9,  1,  7,  4,  9, -1,  0,  0,  0],
    [ 4,  0,  3,  4,  3,  7, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  8,  7, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  9, 10,  8, 10, 11, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0, 11,  3,  0,  9, 11, 11,  9, 10, -1,  0,  0,  0,  0,  0,  0],
    [ 1,  8,  0,  1, 10,  8,  8, 10, 11, -1,  0,  0,  0,  0,  0,  0],
    [ 3,  1, 10,  3, 10, 11, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 2,  9,  1,  2, 11,  9,  9, 11,  8, -1,  0,  0,  0,  0,  0,  0],
    [ 0,  9,  3,  9, 11,  3,  9,  2, 11,  9,  1,  2, -1,  0,  0,  0],
    [11,  8,  0, 11,  0,  2, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 2, 11,  3, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 3, 10,  2,  3,  8, 10, 10,  8,  9, -1,  0,  0,  0,  0,  0,  0],
    [ 9, 10,  2,  9,  2,  0, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 3,  8,  2,  8, 10,  2,  8,  1, 10,  8,  0,  1, -1,  0,  0,  0],
    [ 2,  1, 10, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 8,  9,  1,  8,  1,  3, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 1,  0,  9, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  3,  8, -1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [-1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
];
static EDGE_VERTICES: [[usize; 2]; 12] = [
    [0, 1],
    [1, 2],
    [2, 3],
    [3, 0],
    [4, 5],
    [6, 5],
    [6, 7],
    [7, 4],
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7],
];

fn lerp_verts(va: [u8; 3], vb: [u8; 3], fa: f32, fb: f32, isoval: f32, v: &mut [f32; 3]) {
    let t =
    if f32::abs(fa - fb) < 0.0001 {
        0.0
    } else {
        (isoval - fa) / (fb - fa)
    };
    
    v[0] = va[0] as f32 + (vb[0] as f32 - va[0] as f32) * t;
    v[1] = va[1] as f32 + (vb[1] as f32 - va[1] as f32) * t;
    v[2] = va[2] as f32 + (vb[2] as f32 - va[2] as f32) * t;
}

#[wasm_bindgen]
pub struct MarchingCubes {
    dims: [u32; 3],
    // The input volume, stored on the WASM side
    volume: Vec<u16>, 
    mask: Vec<u8>,
    // The computed triangles, stored on the WASM side
    triangles: Vec<f32>,
    cube: Vec<[u8;3]>
}

impl MarchingCubes {
    // Compute the vertex values of the cell given the ID of its bottom vertex
    fn compute_vertex_values(&self, cell_i: u32, cell_j: u32, cell_k: u32, values: &mut [u8; 8]) {
        for (i, ref v) in self.cube.iter().enumerate() {
            // We want to swap the order we go when on the top of the cube,
            // due to how the indices are labeled in the paper.
            let voxel = ((cell_k + v[2] as u32) * self.dims[1] + cell_j + v[1] as u32) * self.dims[0] + cell_i + v[0] as u32;
            let group = voxel / 8;
            let index = voxel % 8;
            values[i] = (self.mask[group as usize]>>index) & 1;
        };
    }
}

#[wasm_bindgen]
impl MarchingCubes {
    pub fn new() -> MarchingCubes {
        MarchingCubes {
            dims: [0, 0, 0],
            volume: Vec::new(),
            mask: Vec::new(),
            triangles: Vec::new(),
            cube: Vec::new()
        }
    }

    pub fn set_volume(&mut self, volume: Vec<u16>, mask: Vec<u8>, dims_u: u32, dims_v: u32, dims_d: u32) {
        self.volume = volume;
        self.mask = mask;
        
        //self.volume = volume.into_iter().rev().collect();
        self.dims[0] = dims_u ;
        self.dims[1] = dims_v;//cap upper and lower holes
        self.dims[2] = dims_d;
    }

    pub fn set_cube(&mut self, ratio: u8) {
        self.cube.clear();
        self.cube.push([0, 0, 0]);
        self.cube.push([ratio, 0, 0]);
        self.cube.push([ratio, ratio, 0]);
        self.cube.push([0, ratio, 0]);
        self.cube.push([0, 0, ratio]);
        self.cube.push([ratio, 0, ratio]);
        self.cube.push([ratio, ratio, ratio]);
        self.cube.push([0, ratio, ratio]);
    }
     
    // Run the Marching Cubes algorithm on the volume to compute
    // the isosurface at the desired value, and return a reference to the triangle data to JS
    pub fn marching_cubes(&mut self, ratio : u32) -> js_sys::Float32Array {
        self.triangles.clear();
        let mut vals:[u8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut vert = [0.0, 0.0, 0.0];
        let diff_i = (ratio as f32 - self.dims[0] as f32) * 0.5;
        let diff_j = (ratio as f32 - self.dims[1] as f32) * 0.5;
        let diff_k = (ratio as f32 - self.dims[2] as f32) * 0.5;
        self.set_cube(ratio as u8);

        for k in (0..self.dims[2] - ratio).step_by(ratio as usize) {
            for j in (0..self.dims[1] - ratio).step_by(ratio as usize) {
                for i in (0..self.dims[0] - ratio).step_by(ratio as usize)  {
                    self.compute_vertex_values( i, j, k, &mut vals);
                    let mut index = 0;
                    for v in 0..8 {
                        index |= (1 - vals[v]) << v;
                    }

                    /* The cube vertex and edge indices for base rotation:
                        *
                        *      v7------e6------v6
                        *     / |              /|
                        *   e11 |            e10|
                        *   /   e7           /  |
                        *  /    |           /   e5
                        *  v3------e2-------v2  |
                        *  |    |           |   |
                        *  |   v4------e4---|---v5
                        *  e3  /           e1   /
                        *  |  e8            |  e9
                        *  | /              | /    y z
                        *  |/               |/     |/
                        *  v0------e0-------v1     O--x
                        */

                    // The triangle table gives us the mapping from index to actual
                    // triangles to return for this configuration
                    for t in TRI_TABLE[index as usize].iter().take_while(|t| ** t >= 0) {
                        
                        let v_idx = *t as usize;
                        let v0 = EDGE_VERTICES[v_idx][0];
                        let v1 = EDGE_VERTICES[v_idx][1];

                        lerp_verts(self.cube[v0], self.cube[v1],
                            vals[v0] as f32, vals[v1] as f32, 0.5, &mut vert);

                        // Note: The vertex positions need to be placed on the dual grid,
                        // since that's where the isosurface is computed and defined.
                        self.triangles.push(vert[0] + i as f32 + diff_i);
                        self.triangles.push(vert[1] + j as f32 + diff_j);
                        self.triangles.push(vert[2] + k as f32 + diff_k);
                        
                    }
                }
            }
        }
        
        //頂部邊界
        for j in (0..self.dims[1] - ratio).step_by(ratio as usize) {
            for i in (0..self.dims[0] - ratio).step_by(ratio as usize)  {
                self.compute_vertex_values( i, j, 0, &mut vals);
                let mut index = 0;

                vals[4] = vals[0];
                vals[5] = vals[1];
                vals[6] = vals[2];
                vals[7] = vals[3];
                vals[0] = 0;
                vals[1] = 0;
                vals[2] = 0;
                vals[3] = 0;

                for v in 0..8 {
                    index |= (1 - vals[v]) << v;
                }

                for t in TRI_TABLE[index as usize].iter().take_while(|t| ** t >= 0) {
                    
                    let v_idx = *t as usize;
                    let v0 = EDGE_VERTICES[v_idx][0];
                    let v1 = EDGE_VERTICES[v_idx][1];

                    lerp_verts(self.cube[v0], self.cube[v1],
                        vals[v0] as f32, vals[v1] as f32, 0.5, &mut vert);

                    // Note: The vertex positions need to be placed on the dual grid,
                    // since that's where the isosurface is computed and defined.
                    self.triangles.push(vert[0] + i as f32 + diff_i);
                    self.triangles.push(vert[1] + j as f32 + diff_j);
                    self.triangles.push(vert[2] as f32 - ratio as f32 * 0.5 + diff_k);
                    
                }

            }
        }

        //底部邊界
        for j in (0..self.dims[1] - ratio).step_by(ratio as usize) {
            for i in (0..self.dims[0] - ratio).step_by(ratio as usize)  {
                self.compute_vertex_values( i, j, self.dims[2] - ratio * 2, &mut vals);
                let mut index = 0;

                vals[0] = vals[4];
                vals[1] = vals[5];
                vals[2] = vals[6];
                vals[3] = vals[7];
                vals[4] = 0;
                vals[5] = 0;
                vals[6] = 0;
                vals[7] = 0;

                for v in 0..8 {
                    index |= (1 - vals[v]) << v;
                }

                for t in TRI_TABLE[index as usize].iter().take_while(|t| ** t >= 0) {
                    
                    let v_idx = *t as usize;
                    let v0 = EDGE_VERTICES[v_idx][0];
                    let v1 = EDGE_VERTICES[v_idx][1];

                    lerp_verts(self.cube[v0], self.cube[v1],
                        vals[v0] as f32, vals[v1] as f32, 0.5, &mut vert);

                    // Note: The vertex positions need to be placed on the dual grid,
                    // since that's where the isosurface is computed and defined.
                    self.triangles.push(vert[0] + i as f32 + diff_i);
                    self.triangles.push(vert[1] + j as f32 + diff_j);
                    self.triangles.push(vert[2] + (self.dims[2] as f32 - ratio as f32 * 1.5) + diff_k);
                    
                }

            }
        }

        //前側邊界
        for k in (0..self.dims[2] - ratio).step_by(ratio as usize) {
            for i in (0..self.dims[0] - ratio).step_by(ratio as usize)  {
                self.compute_vertex_values( i, 0, k, &mut vals);
                let mut index = 0;

                vals[3] = vals[0];
                vals[2] = vals[1];
                vals[7] = vals[4];
                vals[6] = vals[5];
                vals[0] = 0;
                vals[1] = 0;
                vals[4] = 0;
                vals[5] = 0;

                for v in 0..8 {
                    index |= (1 - vals[v]) << v;
                }

                for t in TRI_TABLE[index as usize].iter().take_while(|t| ** t >= 0) {
                    
                    let v_idx = *t as usize;
                    let v0 = EDGE_VERTICES[v_idx][0];
                    let v1 = EDGE_VERTICES[v_idx][1];

                    lerp_verts(self.cube[v0], self.cube[v1],
                        vals[v0] as f32, vals[v1] as f32, 0.5, &mut vert);

                    // Note: The vertex positions need to be placed on the dual grid,
                    // since that's where the isosurface is computed and defined.
                    self.triangles.push(vert[0] + i as f32 + diff_i);
                    self.triangles.push(vert[1] as f32 - ratio as f32 * 0.5 + diff_j);
                    self.triangles.push(vert[2] + k as f32 + diff_k);
                    
                }

            }
        }

        //後側邊界
        for k in (0..self.dims[2] - ratio).step_by(ratio as usize) {
            for i in (0..self.dims[0] - ratio).step_by(ratio as usize)  {
                self.compute_vertex_values( i, self.dims[1] - ratio * 2, k, &mut vals);
                let mut index = 0;

                vals[0] = vals[3];
                vals[1] = vals[2];
                vals[4] = vals[7];
                vals[5] = vals[6];
                vals[3] = 0;
                vals[2] = 0;
                vals[7] = 0;
                vals[6] = 0;

                for v in 0..8 {
                    index |= (1 - vals[v]) << v;
                }

                for t in TRI_TABLE[index as usize].iter().take_while(|t| ** t >= 0) {
                    
                    let v_idx = *t as usize;
                    let v0 = EDGE_VERTICES[v_idx][0];
                    let v1 = EDGE_VERTICES[v_idx][1];

                    lerp_verts(self.cube[v0], self.cube[v1],
                        vals[v0] as f32, vals[v1] as f32, 0.5, &mut vert);

                    // Note: The vertex positions need to be placed on the dual grid,
                    // since that's where the isosurface is computed and defined.
                    self.triangles.push(vert[0] + i as f32 + diff_i);
                    self.triangles.push(vert[1] + (self.dims[1] as f32 - ratio as f32 * 1.5) + diff_j);
                    self.triangles.push(vert[2] + k as f32 + diff_k);
                    
                }

            }
        }

        //左側邊界
        for k in (0..self.dims[2] - ratio).step_by(ratio as usize) {
            for j in (0..self.dims[1] - ratio).step_by(ratio as usize)  {
                self.compute_vertex_values( 0, j, k, &mut vals);
                let mut index = 0;

                vals[1] = vals[0];
                vals[2] = vals[3];
                vals[5] = vals[4];
                vals[6] = vals[7];
                vals[0] = 0;
                vals[3] = 0;
                vals[4] = 0;
                vals[7] = 0;

                for v in 0..8 {
                    index |= (1 - vals[v]) << v;
                }

                for t in TRI_TABLE[index as usize].iter().take_while(|t| ** t >= 0) {
                    
                    let v_idx = *t as usize;
                    let v0 = EDGE_VERTICES[v_idx][0];
                    let v1 = EDGE_VERTICES[v_idx][1];

                    lerp_verts(self.cube[v0], self.cube[v1],
                        vals[v0] as f32, vals[v1] as f32, 0.5, &mut vert);

                    // Note: The vertex positions need to be placed on the dual grid,
                    // since that's where the isosurface is computed and defined.
                    self.triangles.push(vert[0] as f32 - ratio as f32 * 0.5 + diff_i);
                    self.triangles.push(vert[1] + j as f32 + diff_j);
                    self.triangles.push(vert[2] + k as f32 + diff_k);
                    
                }

            }
        }

        //右側邊界
        for k in (0..self.dims[2] - ratio).step_by(ratio as usize) {
            for j in (0..self.dims[1] - ratio).step_by(ratio as usize)  {
                self.compute_vertex_values( self.dims[0] - ratio * 2, j, k, &mut vals);
                let mut index = 0;

                vals[0] = vals[1];
                vals[3] = vals[2];
                vals[4] = vals[5];
                vals[7] = vals[6];
                vals[1] = 0;
                vals[2] = 0;
                vals[5] = 0;
                vals[6] = 0;

                for v in 0..8 {
                    index |= (1 - vals[v]) << v;
                }

                for t in TRI_TABLE[index as usize].iter().take_while(|t| ** t >= 0) {
                    
                    let v_idx = *t as usize;
                    let v0 = EDGE_VERTICES[v_idx][0];
                    let v1 = EDGE_VERTICES[v_idx][1];

                    lerp_verts(self.cube[v0], self.cube[v1],
                        vals[v0] as f32, vals[v1] as f32, 0.5, &mut vert);

                    // Note: The vertex positions need to be placed on the dual grid,
                    // since that's where the isosurface is computed and defined.
                    self.triangles.push(vert[0] + (self.dims[0]as f32 - ratio as f32 * 1.5) + diff_i);
                    self.triangles.push(vert[1] + j as f32 + diff_j);
                    self.triangles.push(vert[2] + k as f32 + diff_k);
                    
                }

            }
        }


        unsafe { js_sys::Float32Array::view(&self.triangles[..]) }
    }
}

