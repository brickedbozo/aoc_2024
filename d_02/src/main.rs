#![allow(dead_code, unused)]
use std::fs;
fn main() {
    let list = parse_input("i2");
    let p1 = calc_part1(&list);
    let p2 = calc_part2(&list);
    println!("{:?}", p1);
    println!("{:?}", p2);
}
fn calc_part1(l: &[Vec<u64>]) -> u64 {
    let mut res = 0;
    for r in l.iter() {
        if validate_report(r) {
            res += 1;
        }
    }
    res
}
fn calc_part2(l: &[Vec<u64>]) -> u64 {
    let mut res = 0;
    for r in l.iter() {
        if validate_report2(r) {
            res += 1;
        }
    }
    res
}
fn validate_report2(r: &[u64]) -> bool {
    if validate_report(r) {
        return true;
    }
    let mut i = 0;
    while i < r.len() {
        let arr = [&r[..i], &r[i + 1..]].concat();
        if validate_report(&arr) {
            return true;
        }
        i += 1;
    }
    false
}
fn validate_report(r: &[u64]) -> bool {
    if r[0] == r[1] {
        return false;
    }
    match r[0] > r[1] {
        true => {
            let mut i = 0;
            while i < r.len() - 1 {
                let res = r[i] as i64 - r[i + 1] as i64;
                if !(1..=3).contains(&res) {
                    return false;
                }
                i += 1;
            }
        }
        false => {
            let mut i = 0;
            while i < r.len() - 1 {
                let res = r[i + 1] as i64 - r[i] as i64;
                if !(1..=3).contains(&res) {
                    return false;
                }
                i += 1;
            }
        }
    }
    true
}
fn parse_input(path: &str) -> Vec<Vec<u64>> {
    let f = fs::read_to_string(path).unwrap();
    let mut res = Vec::new();
    for l in f.lines() {
        let v: Vec<u64> = l.split_whitespace().map(parse_str_to_number).collect();
        res.push(v);
    }
    res
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
