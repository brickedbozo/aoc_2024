#![allow(dead_code, unused, clippy::all)]
use d_07::*;
use std::fs;
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    println!("part1: {p1}");
    let p2 = calc_part2(&inp);
    println!("part2: {p2}");
}
fn calc_part1(inp: &str) -> u64 {
    let mut answer = 0;
    let equations = parse_input(inp);
    for eq in equations {
        if is_eq_true(&eq) {
            answer += eq.0;
        }
    }
    answer
}
fn calc_part2(inp: &str) -> u64 {
    let mut answer = 0;
    let equations = parse_input(inp);
    for eq in equations {
        if is_eq_true_2(&eq) {
            answer += eq.0;
        }
    }
    answer
}
