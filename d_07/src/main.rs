#![allow(dead_code, unused, clippy::all)]

use std::{
    collections::{HashSet, VecDeque},
    fs,
};
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    println!("part1: {p1}");
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
fn is_eq_true(eq: &(u64, Vec<u64>)) -> bool {
    let test_val = eq.0;
    let nums = &eq.1;
    let variations = get_eq_variations(&nums);
    for var in variations {
        let mut res = nums[0];
        for i in 0..var.len() {
            match var[i] {
                '+' => res += nums[i + 1],
                '*' => res *= nums[i + 1],
                _ => {}
            }
        }
        if res == test_val {
            return true;
        }
    }
    false
}
fn get_eq_variations(nums: &[u64]) -> Vec<VecDeque<char>> {
    let op_count = nums.len() - 1;
    let bit = 2_u32.pow((op_count as u32)) - 1;
    let mut variations = vec![];
    for i in 0..=bit {
        let mut a = format!("{:b}", i).chars().collect::<VecDeque<char>>();
        for i in a.len()..op_count {
            a.push_front('0');
        }
        for c in a.iter_mut() {
            match c {
                '0' => *c = '+',
                '1' => *c = '*',
                _ => *c = 'a',
            }
        }
        variations.push(a);
    }
    variations
}
fn parse_input(inp: &str) -> Vec<(u64, Vec<u64>)> {
    let mut res = vec![];
    for l in inp.lines() {
        let mut it = l.split_terminator(":");
        let cal_res = it.next().unwrap();
        let cal_res = parse_str_to_number(cal_res);
        let numbers = it
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(parse_str_to_number)
            .collect::<Vec<u64>>();
        res.push((cal_res, numbers));
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
