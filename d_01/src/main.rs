#![allow(dead_code, unused)]
use std::{collections::HashMap, fs};

fn main() {
    let inp = parse_input("i2");
    let p1 = part_1(&inp);
    let p2 = part_2(&inp);
    println!("{:?}", p1);
    println!("{:?}", p2);
}
fn part_2(inp: &[Vec<u64>; 2]) -> u64 {
    let mut res = 0;
    let mut hm: HashMap<u64, u64> = HashMap::new();
    for n in inp[1].iter() {
        hm.entry(*n).and_modify(|e| *e += 1).or_insert(1);
    }
    for n in inp[0].iter() {
        let map_res = hm.get(n).unwrap_or(&0);
        res += n * map_res;
    }
    res
}
fn part_1(inp: &[Vec<u64>; 2]) -> u64 {
    let mut res = 0;
    let mut i = 0;
    while i < inp[0].len() {
        if inp[0][i] >= inp[1][i] {
            res += inp[0][i] - inp[1][i];
        } else {
            res += inp[1][i] - inp[0][i];
        }
        i += 1;
    }
    res
}
fn parse_input(path: &str) -> [Vec<u64>; 2] {
    let f = fs::read_to_string(path).unwrap();
    let mut res: [Vec<u64>; 2] = [Vec::new(), Vec::new()];
    for l in f.lines() {
        let (mut num1, mut num2) = (0, 0);
        let mut it = l.split_terminator("   ");
        num1 = parse_str_to_number(it.next().unwrap());
        num2 = parse_str_to_number(it.next().unwrap());
        res[0].push(num1);
        res[1].push(num2);
    }
    res[0].sort();
    res[1].sort();
    res
}
fn parse_str_to_number(s: &str) -> u64 {
    let mut result = 0;
    let mut div = 1;
    for c in s.chars().rev() {
        let n = (c as u8) - b'0';
        result += n as u64 * div;
        div *= 10;
    }
    result
}
