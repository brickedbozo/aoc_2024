#![allow(dead_code, unused, clippy::all)]

use std::{
    collections::{HashSet, VecDeque},
    fs,
};
fn main() {
    let inp = fs::read_to_string("i1").unwrap();
    // let p1 = calc_part1(&inp);
    // println!("{p1}");
    let p2 = calc_part2(&inp);
    println!("{p2}");
}
fn calc_part2(inp: &str) -> usize {
    let (mut pos, grid) = parse_inp(inp);
    let mut pos_set = HashSet::new();
    let start_pos = pos;
    pos_set.insert(pos);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dir = 'n';
    let mut path_len = 0_u64;
    let mut path_vec = VecDeque::with_capacity(4);
    let mut obstacle_set = HashSet::new();
    'main: loop {
        let y = pos.0;
        let x = pos.1;
        if path_vec.len() == 4 {
            path_vec.pop_front();
        }
        match dir {
            'n' => {
                if y > 0 {
                    let is_obstacle = grid[y - 1][x];
                    if is_obstacle {
                        path_vec.push_back(path_len);
                        path_len = 0;
                        dir = 'e';
                    } else {
                        path_len += 1;
                        pos = (y - 1, x);
                        pos_set.insert(pos);
                        continue;
                    }
                } else {
                    break;
                }
            }
            'e' => {
                if x < cols - 1 {
                    let is_obstacle = grid[y][x + 1];
                    if is_obstacle {
                        path_vec.push_back(path_len);
                        path_len = 0;
                        dir = 's';
                    } else {
                        path_len += 1;
                        pos = (y, x + 1);
                        pos_set.insert(pos);
                        continue;
                    }
                } else {
                    break;
                }
            }
            's' => {
                if y < rows - 1 {
                    let is_obstacle = grid[y + 1][x];
                    if is_obstacle {
                        path_vec.push_back(path_len);
                        path_len = 0;
                        dir = 'w';
                    } else {
                        path_len += 1;
                        pos = (y + 1, x);
                        pos_set.insert(pos);
                        continue;
                    }
                } else {
                    break;
                }
            }
            'w' => {
                if x > 0 {
                    let is_obstacle = grid[y][x - 1];
                    if is_obstacle {
                        path_vec.push_back(path_len);
                        path_len = 0;
                        dir = 'n';
                    } else {
                        path_len += 1;
                        pos = (y, x - 1);
                        pos_set.insert(pos);
                        continue;
                    }
                } else {
                    break;
                }
            }
            _ => {}
        }
        if path_vec.len() == 4 {
            // println!("{:?} {:?}", path_vec, dir);
            // if path_vec[0] >= path_vec[2] && path_vec[1] < path_vec[3] {
            //     println!("{:?}", "DOG");
            // }
            println!("{:?}", path_vec);
            if path_vec[0] >= path_vec[2] && path_vec[1] < path_vec[3] {
                let a = path_vec[3] - path_vec[1];
                let mut is_at_start = true;
                let mut pos_obstacle = (0, 0);
                match dir {
                    'n' => {
                        //last_dir W
                        println!("N{:?} xy:{y}{x} ", path_vec[path_vec.len() - 1]);
                        if (y, x + a as usize - 1) != start_pos {
                            pos_obstacle = (y, x + a as usize - 1);
                            is_at_start = false;
                        }
                        println!("{:?}", pos_obstacle);
                    }
                    'e' => {
                        //last_dir N
                        println!("E{:?}", path_vec[path_vec.len() - 1]);
                        if (y + a as usize - 1, x) != start_pos {
                            pos_obstacle = (y + a as usize - 1, x);
                            is_at_start = false;
                        }
                        println!("{:?}", pos_obstacle);
                    }
                    's' => {
                        //last_dir E
                        println!("S");
                        if (y, x - a as usize + 1) != start_pos {
                            pos_obstacle = (y, x - a as usize + 1);
                            is_at_start = false;
                        }
                        println!("{:?}", pos_obstacle);
                    }
                    'w' => {
                        //last_dir S
                        println!("w");
                        if (y - a as usize + 1, x) != start_pos {
                            pos_obstacle = (y - a as usize + 1, x);
                            is_at_start = false;
                        }
                        println!("{:?}", pos_obstacle);
                    }
                    _ => {}
                }
                if !is_at_start {
                    obstacle_set.insert(pos_obstacle);
                }
            }
        }
    }
    println!("{:?}", obstacle_set);
    pos_set.len()
}
fn calc_part1(inp: &str) -> usize {
    let (mut pos, grid) = parse_inp(inp);
    let mut pos_set = HashSet::new();
    pos_set.insert(pos);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dir = 'n';
    'main: loop {
        let y = pos.0;
        let x = pos.1;
        match dir {
            'n' => {
                if y > 0 {
                    let is_obstacle = grid[y - 1][x];
                    if is_obstacle {
                        dir = 'e';
                    } else {
                        pos = (y - 1, x);
                        pos_set.insert(pos);
                    }
                } else {
                    break;
                }
            }
            'e' => {
                if x < cols - 1 {
                    let is_obstacle = grid[y][x + 1];
                    if is_obstacle {
                        dir = 's';
                    } else {
                        pos = (y, x + 1);
                        pos_set.insert(pos);
                    }
                } else {
                    break;
                }
            }
            's' => {
                if y < rows - 1 {
                    let is_obstacle = grid[y + 1][x];
                    if is_obstacle {
                        dir = 'w';
                    } else {
                        pos = (y + 1, x);
                        pos_set.insert(pos);
                    }
                } else {
                    break;
                }
            }
            'w' => {
                if x > 0 {
                    let is_obstacle = grid[y][x - 1];
                    if is_obstacle {
                        dir = 'n';
                    } else {
                        pos = (y, x - 1);
                        pos_set.insert(pos);
                    }
                } else {
                    break;
                }
            }
            _ => {}
        }
    }
    pos_set.len()
}
fn parse_inp(inp: &str) -> ((usize, usize), Vec<Vec<bool>>) {
    let mut start_pos = (0, 0);
    let mut i = 0;
    let mut j = 0;
    let rows = inp.lines().count();
    let cols = inp.lines().next().unwrap().chars().count();
    let mut grid = Vec::with_capacity(rows);
    let mut row = Vec::with_capacity(cols);
    let inp = inp.replace("\n", "");
    for c in inp.chars() {
        match c {
            '.' => row.push(false),
            '#' => row.push(true),
            _ => {
                start_pos = (i, j);
                row.push(false);
            }
        }
        j += 1;
        if j == cols {
            grid.push(row);
            row = Vec::with_capacity(cols);
            j = 0;
            i += 1;
        }
    }
    (start_pos, grid)
}
