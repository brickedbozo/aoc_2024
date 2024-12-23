use d_06::*;
use std::{collections::HashSet, fs};

fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    println!("part1: {p1}");
    let p2 = calc_part2(&inp);
    println!("part2: {p2}");
}
fn calc_part1(inp: &str) -> usize {
    let inp = parse_inp(inp);
    let mut hs = HashSet::new();
    inp.get_path().into_iter().for_each(|e| {
        hs.insert(e);
    });
    hs.len()
}

fn calc_part2(inp: &str) -> usize {
    let mut inp = parse_inp(inp);
    inp.get_obstacle_loop_count()
}
