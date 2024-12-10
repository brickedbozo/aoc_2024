#![allow(dead_code, unused, clippy::all)]
use std::{collections::HashSet, fs};
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_p1(&inp);
    println!("part1: {p1}");
    let p2 = calc_p2(&inp);
    println!("part2: {p2}");
}
fn calc_p2(inp: &str) -> u64 {
    let mut res = 0;
    let mut disk = parse_input(inp);
    sort_disk2(&mut disk);
    let mut res = 0;
    disk.into_iter().enumerate().for_each(|(i, e)| match e {
        Some(n) => {
            res += i as u64 * n;
        }
        None => {}
    });
    res
}
fn sort_disk2(disk: &mut [Option<u64>]) {
    let mut tail = disk.len() - 1;
    while tail > 0 {
        match disk[tail] {
            Some(id) => {
                let mut id_len = 1;
                for j in (0..tail).rev() {
                    if let Some(n) = disk[j] {
                        if n == id {
                            id_len += 1;
                            tail -= 1;
                            continue;
                        }
                    }
                    break;
                }
                let mut dot_len = 0;
                for start in 0..tail {
                    if disk[start].is_none() {
                        dot_len += 1;
                    } else {
                        dot_len = 0;
                        continue;
                    }
                    if dot_len == id_len {
                        for k in start - dot_len + 1..=start {
                            disk[k] = Some(id);
                        }
                        for k in tail..=tail + id_len - 1 {
                            disk[k] = None;
                        }
                        break;
                    }
                }
            }
            None => {}
        }
        if tail > 0 {
            tail -= 1;
        } else {
            break;
        }
    }
}
fn calc_p1(inp: &str) -> u64 {
    let mut res = 0;
    let mut disk = parse_input(inp);
    sort_disk(&mut disk);
    let last_num_loc = disk.iter().rposition(|&e| e != None).unwrap();
    let disk = &disk[0..=last_num_loc];
    disk.into_iter()
        .map(|&e| e.unwrap())
        .enumerate()
        .for_each(|(i, n)| {
            res += i as u64 * n;
        });
    res
}
fn sort_disk(disk: &mut [Option<u64>]) {
    let dl = disk.len();
    for i in 0..dl {
        if disk[i].is_none() {
            for j in (i + 1..dl).rev() {
                if let Some(n) = disk[j] {
                    disk[i] = Some(n);
                    disk[j] = None;
                    break;
                }
            }
        }
    }
}
fn parse_input(inp: &str) -> Vec<Option<u64>> {
    let inp = &inp[..inp.len() - 1];
    let mut res = vec![];
    let mut id: u64 = 0;
    for (i, c) in inp.chars().enumerate() {
        let n = c as u8 - b'0';
        if i % 2 == 0 {
            for _ in 0..n {
                res.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..n {
                res.push(None);
            }
        }
    }
    res
}
