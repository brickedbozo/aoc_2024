#![allow(dead_code, unused, clippy::all)]
use std::{
    collections::{HashMap, HashSet},
    fs,
};
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point(usize, usize);
type Grid = Vec<Vec<Option<u8>>>;
impl Point {
    fn find_nines(&self, grid: &Grid) -> HashSet<Point> {
        let mut res = HashSet::new();
        let row = self.0;
        let col = self.1;
        let cols = grid[0].len();
        let rows = grid.len();
        // println!("{:?}", self);
        if let Some(n) = grid[row][col] {
            if n == 9 {
                res.insert(Point(row, col));
                return res;
            }
            //T
            if row > 0 {
                if let Some(next) = grid[row - 1][col] {
                    if next > n && next - n == 1 {
                        let top_point = Point(row - 1, col);
                        for p in top_point.find_nines(&grid) {
                            res.insert(p);
                        }
                    }
                }
            }
            //R
            if col < cols - 1 {
                if let Some(next) = grid[row][col + 1] {
                    if next > n && next - n == 1 {
                        let right_point = Point(row, col + 1);
                        for p in right_point.find_nines(&grid) {
                            res.insert(p);
                        }
                    }
                }
            }
            //B
            if row < rows - 1 {
                if let Some(next) = grid[row + 1][col] {
                    if next > n && next - n == 1 {
                        let bottom_point = Point(row + 1, col);
                        for p in bottom_point.find_nines(&grid) {
                            res.insert(p);
                        }
                    }
                }
            }
            //L
            if col > 0 {
                if let Some(next) = grid[row][col - 1] {
                    if next > n && next - n == 1 {
                        let left_point = Point(row, col - 1);
                        for p in left_point.find_nines(&grid) {
                            res.insert(p);
                        }
                    }
                }
            }
        }
        res
    }
    fn find_nines_rating(&self, grid: &Grid) -> Vec<HashSet<Point>> {
        let mut res = vec![];
        let row = self.0;
        let col = self.1;
        let cols = grid[0].len();
        let rows = grid.len();
        if let Some(n) = grid[row][col] {
            if n == 9 {
                let mut hs = HashSet::new();
                hs.insert(*self);
                res.push(hs);
                return res;
            }
            //T
            if row > 0 {
                if let Some(next) = grid[row - 1][col] {
                    if next > n && next - n == 1 {
                        let top_point = Point(row - 1, col);
                        let v = top_point.find_nines_rating(&grid);
                        for mut s in v {
                            s.insert(*self);
                            res.push(s);
                        }
                    }
                }
            }
            //R
            if col < cols - 1 {
                if let Some(next) = grid[row][col + 1] {
                    if next > n && next - n == 1 {
                        let right_point = Point(row, col + 1);
                        let v = right_point.find_nines_rating(&grid);
                        for mut s in v {
                            s.insert(*self);
                            res.push(s);
                        }
                    }
                }
            }
            //B
            if row < rows - 1 {
                if let Some(next) = grid[row + 1][col] {
                    if next > n && next - n == 1 {
                        let bottom_point = Point(row + 1, col);
                        let v = bottom_point.find_nines_rating(&grid);
                        for mut s in v {
                            s.insert(*self);
                            res.push(s);
                        }
                    }
                }
            }
            //L
            if col > 0 {
                if let Some(next) = grid[row][col - 1] {
                    if next > n && next - n == 1 {
                        let left_point = Point(row, col - 1);
                        let v = left_point.find_nines_rating(&grid);
                        for mut s in v {
                            s.insert(*self);
                            res.push(s);
                        }
                    }
                }
            }
        }
        res
    }
}
fn main() {
    let inp = fs::read_to_string("i1").unwrap();
    let p1 = calc_p1(&inp);
    println!("part1: {p1}");
    let p2 = calc_p2(&inp);
    println!("part2: {p2}");
}
fn calc_p1(inp: &str) -> usize {
    let mut res = 0;
    let (start_pos, grid) = parse_input(inp);
    for pos in start_pos {
        let h = pos.find_nines(&grid);
        res += h.len();
    }
    res
}
fn calc_p2(inp: &str) -> usize {
    let mut res = 0;
    let (start_pos, grid) = parse_input(inp);
    for pos in start_pos {
        let v = pos.find_nines_rating(&grid);
        for h in v {
            res += 1;
        }
    }
    res
}
fn parse_input(inp: &str) -> (Vec<Point>, Grid) {
    let mut start_pos = vec![];
    let mut grid = vec![];
    for (i, line) in inp.lines().enumerate() {
        grid.push(Vec::with_capacity(line.chars().count()));
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let n = c as u8 - b'0';
                if n == 0 {
                    start_pos.push(Point(i, j));
                }
                grid[i].push(Some(n));
            } else {
                grid[i].push(None);
            }
        }
    }
    (start_pos, grid)
}
