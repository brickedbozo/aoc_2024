#![allow(dead_code, unused, clippy::all)]
use std::fs;
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_p1(&inp);
    println!("part1: {p1}");
    // let p2 = calc_p2(&inp);
    // println!("part2: {p2}");
}
fn calc_p1(inp: &str) -> usize {
    let mut res = 0;
    let mut stones = parse_input(inp);
    for _ in 0..25 {
        let mut v = vec![];
        for &stone in &stones {
            v.append(&mut blink(stone));
        }
        stones = v;
    }
    stones.len()
}
fn calc_p2(inp: &str) -> usize {
    let mut res = 0;
    let mut stones = parse_input(inp);
    res
}
fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (l, r) = stone_str.split_at(stone_str.len() / 2);
        let l = parse_str_to_number(l);
        let r = parse_str_to_number(r);
        return vec![l, r];
    }
    vec![stone * 2024]
}
fn parse_input(inp: &str) -> Vec<u64> {
    inp.split_whitespace()
        .map(|e| parse_str_to_number(e))
        .collect::<Vec<u64>>()
}
fn parse_str_to_number(s: &str) -> u64 {
    let mut res = 0;
    let mut div = 1;
    for c in s.chars().rev() {
        let n = (c as u8) - b'0';
        res += n as u64 * div;
        div *= 10;
    }
    res
}
