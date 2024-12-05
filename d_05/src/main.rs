#![allow(dead_code, unused, clippy::all)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    println!("{p1}");
    let p2 = calc_part2(&inp);
    println!("{p2}");
}
fn calc_part2(inp: &str) -> u64 {
    let mut res = 0;
    let (map, nums_list) = parse_input(inp);
    let mut nums_list = get_incorrectly_ordered(&nums_list, &map);
    for nums in &mut nums_list {
        while !check_if_ordered(&nums, &map) {
            let mut i = 0;
            while i < nums.len() - 1 {
                let l = nums[i];
                let r = nums[i + 1];
                let a = check_if_ordered(&[l, r], &map);
                if !a {
                    nums[i] = r;
                    nums[i + 1] = l;
                }
                i += 1;
            }
        }
        let mid = nums.len() / 2;
        res += nums[mid];
    }
    res
}
fn get_incorrectly_ordered(nums_list: &[Vec<u64>], map: &HashMap<u64, Vec<u64>>) -> Vec<Vec<u64>> {
    let mut res = vec![];
    for nums in nums_list {
        if !check_if_ordered(nums, &map) {
            res.push(nums.clone());
        }
    }
    res
}
fn calc_part1(inp: &str) -> u64 {
    let mut res = 0;
    let (mut map, mut nums_list) = parse_input(inp);
    'main: for nums in nums_list {
        let a = check_if_ordered(&nums, &map);
        if a {
            let mid = nums.len() / 2;
            res += nums[mid];
        }
    }
    res
}
fn check_if_ordered(nums: &[u64], map: &HashMap<u64, Vec<u64>>) -> bool {
    let mut res = 0;
    let mut visited = HashSet::new();
    for num in nums {
        if let Some(map_nums) = map.get(&num) {
            for n in map_nums {
                if visited.contains(n) {
                    return false;
                }
            }
        };
        visited.insert(num);
    }
    true
}
fn parse_input(inp: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let mut empty_idx = 0;
    for (i, l) in inp.lines().enumerate() {
        if l.is_empty() {
            empty_idx = i + 1;
            break;
        }
    }
    let mut i2 = inp.split_terminator("\n\n");
    let p1 = i2.next().unwrap();
    let p2 = i2.next().unwrap();
    let mut map = HashMap::new();
    for l in p1.lines() {
        let left = &l[0..2];
        let right = &l[3..];
        let left = parse_str_to_number(left);
        let right = parse_str_to_number(right);
        map.entry(left)
            .and_modify(|e: &mut Vec<u64>| e.push(right))
            .or_insert(Vec::from([right]));
    }
    let mut numbers = vec![];
    for l in p2.lines() {
        let nums = l
            .split_terminator(',')
            .map(parse_str_to_number)
            .collect::<Vec<u64>>();
        numbers.push(nums);
    }
    (map, numbers)
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
