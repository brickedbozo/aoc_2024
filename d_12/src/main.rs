#![allow(dead_code, unused, clippy::all)]
use std::{
    collections::{HashMap, HashSet},
    fs,
};
type Grid = Vec<Vec<char>>;
type Plot = Vec<Vec<bool>>;
type Pos = (usize, usize);
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    println!("part1: {p1}");
}
fn calc_part1(inp: &str) -> usize {
    let mut res = 0;
    let grid = parse_input(inp);
    let regions = get_regions(&grid);
    for region in regions {
        let plot = get_plot(&region);
        let mut area = 0;
        for row in &plot {
            for col in row {
                if *col {
                    area += 1;
                }
            }
        }
        let mut perimeter = get_plot_perimeter(&plot);
        res += area * perimeter;
        // regions.iter().for_each(|e| println!("{:?}", e));
    }

    res
}
fn get_plot((c, v): &(char, Vec<(usize, usize)>)) -> Plot {
    let mut rows = 0;
    let mut cols = 0;
    for (row, col) in v.iter() {
        if *row > rows {
            rows = *row;
        }
        if *col > cols {
            cols = *col;
        }
    }
    rows += 1;
    cols += 1;
    let mut res = vec![vec![false; cols]; rows];
    for (row, col) in v.iter() {
        res[*row][*col] = true;
    }

    res
}
fn get_regions(grid: &Grid) -> Vec<(char, Vec<Pos>)> {
    let mut res = vec![];
    let mut visited_rec = HashSet::new();
    let mut visited = HashSet::new();
    for (row, v) in grid.iter().enumerate() {
        for (col, &c) in v.iter().enumerate() {
            if !visited.contains(&(row, col)) {
                let hs = get_region(c, (row, col), &grid, &mut visited_rec).unwrap();
                visited.extend(hs.clone());
                let mut set_vec = vec![];
                for s in hs {
                    set_vec.push(s);
                }
                set_vec.sort();
                res.push((c, set_vec));
            }
        }
    }
    res
}
fn get_region(
    c: char,
    (row, col): Pos,
    grid: &Grid,
    visited: &mut HashSet<Pos>,
) -> Option<HashSet<Pos>> {
    if grid[row][col] != c || visited.contains(&(row, col)) {
        return None;
    }
    let mut hs = HashSet::from([(row, col)]);
    visited.insert((row, col));
    let rows = grid.len();
    let cols = grid[0].len();
    if row > 0 {
        let res = get_region(c, (row - 1, col), grid, visited);
        if let Some(s) = res {
            hs.extend(s);
        }
    }
    if col > 0 {
        let res = get_region(c, (row, col - 1), grid, visited);
        if let Some(s) = res {
            hs.extend(s);
        }
    }
    if row < rows - 1 {
        let res = get_region(c, (row + 1, col), grid, visited);
        if let Some(s) = res {
            hs.extend(s);
        }
    }
    if col < cols - 1 {
        let res = get_region(c, (row, col + 1), grid, visited);
        if let Some(s) = res {
            hs.extend(s);
        }
    }
    Some(hs)
}
fn get_plot_perimeter(plot: &Plot) -> usize {
    let mut res = 0;
    let rows = plot.len();
    let cols = plot[0].len();
    // T-B
    for col in 0..cols {
        let mut a = false;
        for row in 0..rows {
            if plot[row][col] && a == false {
                a = true;
                res += 1;
            } else if !plot[row][col] && a == true {
                a = false;
            }
        }
    }
    //B-T
    for col in 0..cols {
        let mut a = false;
        for row in (0..rows).rev() {
            if plot[row][col] && a == false {
                a = true;
                res += 1;
            } else if !plot[row][col] && a == true {
                a = false;
            }
        }
    }
    //L-R
    for row in 0..rows {
        let mut a = false;
        for col in 0..cols {
            if plot[row][col] && a == false {
                a = true;
                res += 1;
            } else if !plot[row][col] && a == true {
                a = false;
            }
        }
    }
    //R-L
    for row in 0..rows {
        let mut a = false;
        for col in (0..cols).rev() {
            if plot[row][col] && a == false {
                a = true;
                res += 1;
            } else if !plot[row][col] && a == true {
                a = false;
            }
        }
    }
    res
}
fn parse_input(inp: &str) -> Grid {
    let mut res = vec![];
    for (row, l) in inp.lines().enumerate() {
        res.push(vec![]);
        l.chars().for_each(|c| res[row].push(c));
    }
    res
}
