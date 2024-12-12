#![allow(dead_code, unused, clippy::all)]
use std::{collections::HashMap, fs};
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_p1(&inp);
    println!("part1: {p1}");
    let p2 = calc_p2(&inp);
    println!("part2: {p2}");
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
    let mut map: HashMap<(u64, usize), usize> = HashMap::new();
    for stone in stones {
        res += blink_rec(stone, 75, &mut map);
    }
    res
}
fn blink_rec(stone: u64, blink_count: usize, map: &mut HashMap<(u64, usize), usize>) -> usize {
    if blink_count == 1 {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            return 2;
        }
        return 1;
    }
    let r = map.get(&(stone, blink_count));
    if let Some(&n) = r {
        return n;
    }
    if stone == 0 {
        let r = blink_rec(1, blink_count - 1, map);
        map.insert((0, blink_count), r);
        return r;
    }
    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (l, r) = stone_str.split_at(stone_str.len() / 2);
        let l = parse_str_to_number(l);
        let r = parse_str_to_number(r);
        let lr = blink_rec(l, blink_count - 1, map);
        map.insert((l, blink_count - 1), lr);
        let rr = blink_rec(r, blink_count - 1, map);
        map.insert((r, blink_count - 1), rr);
        return lr + rr;
    }
    let r = blink_rec(stone * 2024, blink_count - 1, map);
    map.insert((stone, blink_count), r);
    return r;
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
