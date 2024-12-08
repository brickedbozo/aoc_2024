#![allow(dead_code, unused, clippy::all)]

use std::{
    collections::{hash_set, HashMap, HashSet, VecDeque},
    fs,
};
fn main() {
    let inp = fs::read_to_string("i1").unwrap();
    let p1 = calc_part1(&inp);
    println!("{p1}");
    let p2 = calc_part2(&inp);
    println!("{p2}");
}
fn calc_part2(inp: &str) -> usize {
    let (mut start_pos, grid) = parse_inp(inp);
    let path = get_path(start_pos, &grid);
    let mut turn_count = 0;
    let mut side_len = 0;
    let mut pos = start_pos;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dir = 'n';
    let mut obstacle_set = HashSet::new();
    // println!("{:?}", path);
    'main: loop {
        let y = pos.0;
        let x = pos.1;
        match dir {
            'n' => {
                if y > 0 {
                    let is_obstacle = grid[y - 1][x];
                    if is_obstacle {
                        dir = 'e';
                        turn_count += 1;
                        side_len = 0;
                    } else {
                        pos = (y - 1, x);
                        side_len += 1;
                        if turn_count > 2 && pos != start_pos && side_len > 1 {
                            for i in x + 1..cols {
                                let is_obstacle = grid[y][i];
                                if !is_obstacle {
                                    let a = path.get(&(y, i));
                                    if let Some(dirs) = a {
                                        if dirs.contains(&'e') {
                                            obstacle_set.insert(pos);
                                            continue 'main;
                                        } else if dirs.contains(&'s') {
                                            if i + 1 < cols {
                                                let is_obstacle = grid[y][i + 1];
                                                if is_obstacle {
                                                    obstacle_set.insert(pos);
                                                    continue 'main;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
                        turn_count += 1;
                        side_len = 0;
                    } else {
                        pos = (y, x + 1);
                        side_len += 1;
                        if turn_count > 2 && pos != start_pos && side_len > 1 {
                            for i in y + 1..rows {
                                let is_obstacle = grid[i][x];
                                if !is_obstacle {
                                    let a = path.get(&(i, x));
                                    if let Some(dirs) = a {
                                        if dirs.contains(&'s') {
                                            obstacle_set.insert(pos);
                                            continue 'main;
                                        } else if dirs.contains(&'w') {
                                            if i + 1 < rows {
                                                let is_obstacle = grid[i + 1][x];
                                                if is_obstacle {
                                                    obstacle_set.insert(pos);
                                                    continue 'main;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
                        turn_count += 1;
                        side_len = 0;
                    } else {
                        pos = (y + 1, x);
                        side_len += 1;
                        if turn_count > 2 && pos != start_pos && side_len > 1 {
                            for i in (0..x).rev() {
                                let is_obstacle = grid[y][i];
                                if !is_obstacle {
                                    let a = path.get(&(y, i));
                                    if let Some(dirs) = a {
                                        if dirs.contains(&'w') {
                                            obstacle_set.insert(pos);
                                            continue 'main;
                                        } else if dirs.contains(&'n') {
                                            if i > 0 {
                                                let is_obstacle = grid[y][i - 1];
                                                if is_obstacle {
                                                    obstacle_set.insert(pos);
                                                    continue 'main;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
                        side_len = 0;
                    } else {
                        pos = (y, x - 1);
                        side_len += 1;
                        if turn_count > 2 && pos != start_pos && side_len > 1 {
                            for i in (0..y).rev() {
                                let is_obstacle = grid[i][x];
                                if !is_obstacle {
                                    let a = path.get(&(i, x));
                                    if let Some(dirs) = a {
                                        if dirs.contains(&'n') {
                                            obstacle_set.insert(pos);
                                            continue 'main;
                                        } else if dirs.contains(&'e') {
                                            if i > 0 {
                                                let is_obstacle = grid[i - 1][x];
                                                if is_obstacle {
                                                    obstacle_set.insert(pos);
                                                    continue 'main;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            _ => {}
        }
    }
    // println!("{:?}", obstacle_set);
    obstacle_set.len()
}
fn get_path(
    start_pos: (usize, usize),
    grid: &[Vec<bool>],
) -> HashMap<(usize, usize), HashSet<char>> {
    let mut map = HashMap::new();
    let mut dir = 'n';
    map.insert(start_pos, HashSet::from([dir]));
    let rows = grid.len();
    let cols = grid[0].len();
    let mut pos = start_pos;
    'main: loop {
        let y = pos.0;
        let x = pos.1;
        match dir {
            'n' => {
                if y > 0 {
                    let is_obstacle = grid[y - 1][x];
                    if is_obstacle {
                        dir = 'e';
                        map.entry(pos).and_modify(|e| {
                            e.remove(&'n');
                            e.insert(dir);
                        });
                    } else {
                        pos = (y - 1, x);
                        map.entry(pos)
                            .and_modify(|e| {
                                e.insert(dir);
                            })
                            .or_insert(HashSet::from([dir]));
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
                        map.entry(pos).and_modify(|e| {
                            e.remove(&'e');
                            e.insert(dir);
                        });
                    } else {
                        pos = (y, x + 1);
                        map.entry(pos)
                            .and_modify(|e| {
                                e.insert(dir);
                            })
                            .or_insert(HashSet::from([dir]));
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
                        map.entry(pos).and_modify(|e| {
                            e.remove(&'s');
                            e.insert(dir);
                        });
                    } else {
                        pos = (y + 1, x);
                        map.entry(pos)
                            .and_modify(|e| {
                                e.insert(dir);
                            })
                            .or_insert(HashSet::from([dir]));
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
                        map.entry(pos).and_modify(|e| {
                            e.remove(&'w');
                            e.insert(dir);
                        });
                    } else {
                        pos = (y, x - 1);
                        map.entry(pos)
                            .and_modify(|e| {
                                e.insert(dir);
                            })
                            .or_insert(HashSet::from([dir]));
                    }
                } else {
                    break;
                }
            }
            _ => {}
        }
    }
    map
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
