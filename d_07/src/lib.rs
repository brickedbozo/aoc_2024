#![allow(dead_code, unused, clippy::all)]
use std::collections::VecDeque;

pub fn is_eq_true_2(eq: &(u64, Vec<u64>)) -> bool {
    let test_val = eq.0;
    let nums = &eq.1;
    if nums.len() == 1 {
        if test_val == nums[0] {
            return true;
        } else {
            return false;
        }
    }
    let variations = get_eq_variations_2(&nums);
    for var in variations {
        let mut res = nums[0];
        for i in 0..var.len() {
            match var[i] {
                '+' => res += nums[i + 1],
                '*' => res *= nums[i + 1],
                '|' => {
                    let tmp = (res.to_string() + &nums[i + 1].to_string())
                        .parse::<u64>()
                        .unwrap();
                    res = tmp;
                }
                _ => {}
            }
        }
        if res == test_val {
            return true;
        }
    }
    false
}
pub fn is_eq_true(eq: &(u64, Vec<u64>)) -> bool {
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
pub fn get_eq_variations_2(nums: &[u64]) -> Vec<VecDeque<char>> {
    let operator_count = nums.len() - 1;
    let variations_len = 3_usize.pow(operator_count as u32);
    let mut variations = Vec::with_capacity(variations_len);
    for i in 0..variations_len {
        let mut a = format_radix(i as u32, 3)
            .chars()
            .collect::<VecDeque<char>>();
        for i in a.len()..operator_count {
            a.push_front('0');
        }
        for c in a.iter_mut() {
            match c {
                '0' => *c = '+',
                '1' => *c = '*',
                '2' => *c = '|',
                _ => {}
            }
        }
        variations.push(a);
    }
    variations
}
pub fn get_eq_variations(nums: &[u64]) -> Vec<VecDeque<char>> {
    let operator_count = nums.len() - 1;
    let variations_len = 2_usize.pow(operator_count as u32);
    let mut variations = Vec::with_capacity(variations_len);
    for i in 0..variations_len {
        let mut a = format!("{:b}", i).chars().collect::<VecDeque<char>>();
        for i in a.len()..operator_count {
            a.push_front('0');
        }
        for c in a.iter_mut() {
            match c {
                '0' => *c = '+',
                '1' => *c = '*',
                _ => {}
            }
        }
        variations.push(a);
    }
    variations
}
pub fn parse_input(inp: &str) -> Vec<(u64, Vec<u64>)> {
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
pub fn parse_str_to_number(s: &str) -> u64 {
    let mut res = 0;
    let mut div = 1;
    for c in s.chars().rev() {
        let n = (c as u8) - b'0';
        res += n as u64 * div;
        div *= 10;
    }
    res
}
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];
    loop {
        let m = x % radix;
        x = x / radix;
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
