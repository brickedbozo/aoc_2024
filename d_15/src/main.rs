#![allow(dead_code, unused, clippy::all)]
use d_15::*;
use std::fs;
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    println!("part1: {p1}");
}
fn calc_part1(inp: &str) -> usize {
    let mut res = 0;
    let inp = parse_input(inp);
    let grid = traverse_grid(inp);
    for (i, row) in grid.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if let Some(wh) = col {
                if let Warehouse::Bx = wh {
                    res += calc_gps((i, j));
                }
            }
        }
    }
    res
}
