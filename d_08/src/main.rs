#![allow(dead_code, unused, clippy::all)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    println!("part1: {p1}");
}
fn calc_part1(inp: &str) -> u64 {
    let mut res: HashSet<(usize, usize)> = HashSet::new();
    let width = inp.lines().next().unwrap().chars().count();
    let height = inp.lines().count();
    let antennas_map = parse_input(inp);
    // println!("{:?}", anthenas);
    for (_, ants) in &antennas_map {
        for ant1 in ants {
            for ant2 in ants {
                if ant1 == ant2 {
                    continue;
                }
                let (y1, x1) = *ant1;
                let (y2, x2) = *ant2;
                let dist_y = (y1 as i32) - (y2 as i32);
                let dist_x = (x1 as i32) - (x2 as i32);
                let potent_y = (y2 as i32) - dist_y;
                let potent_x = (x2 as i32) - dist_x;
                if potent_y >= 0 && potent_y < height as i32 {
                    if potent_x >= 0 && potent_x < width as i32 {
                        res.insert((potent_y as usize, potent_x as usize));
                    }
                }
                //N
            }
        }
    }
    // println!("{:?}", res);
    res.len() as u64
}
fn parse_input(inp: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut res = HashMap::new();
    for (i, l) in inp.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '.' => continue,
                _ => res
                    .entry(c)
                    .and_modify(|e: &mut Vec<_>| e.push((i, j)))
                    .or_insert(vec![(i, j)]),
            };
        }
    }
    res
}
