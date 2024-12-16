#![allow(dead_code, unused, clippy::all)]
pub type Grid = Vec<Vec<Option<Warehouse>>>;
#[derive(Debug)]
pub enum Direction {
    T,
    B,
    L,
    R,
}
#[derive(Debug, Clone, Copy)]
pub enum Warehouse {
    Bx,
    Wall,
}
#[derive(Debug)]
pub struct Input {
    pub grid: Grid,
    pub start_pos: (usize, usize),
    pub directions: Vec<Direction>,
}
use {Direction::*, Warehouse::*};
pub fn calc_gps(pos: (usize, usize)) -> usize {
    pos.0 * 100 + pos.1
}
pub fn print_grid(grid: &Grid) {
    for row in grid {
        let mut line = String::new();
        for col in row {
            match col {
                Some(e) => match e {
                    Bx => line.push('O'),
                    Wall => line.push('#'),
                },
                None => line.push('.'),
            }
        }
        println!("{:?}", line);
    }
}
pub fn parse_input(inp: &str) -> Input {
    let mut iter = inp.split_terminator("\n\n");
    let width = inp.lines().next().unwrap().chars().count();
    let height = iter.clone().next().unwrap().lines().count();
    let mut grid = vec![vec![None; width]; height];
    let mut start_pos = (0, 0);
    let mut directions = vec![];
    iter.next().unwrap().lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '#' => grid[i][j] = Some(Wall),
            'O' => grid[i][j] = Some(Bx),
            '@' => start_pos = (i, j),
            _ => {}
        });
    });
    iter.next()
        .unwrap()
        .replace("\n", "")
        .chars()
        .for_each(|c| match c {
            '^' => directions.push(T),
            'v' => directions.push(B),
            '<' => directions.push(L),
            '>' => directions.push(R),
            _ => {}
        });
    Input {
        grid,
        start_pos,
        directions,
    }
}
pub fn traverse_grid(inp: Input) -> Grid {
    let mut grid = inp.grid;
    let directions = inp.directions;
    let (mut row, mut col) = inp.start_pos;
    let cols = grid[0].len();
    let rows = grid.len();
    'main: for dir in directions {
        match dir {
            T => {
                if row > 0 {
                    match grid[row - 1][col] {
                        Some(wh) => match wh {
                            Bx => {
                                if row > 1 {
                                    for j in (0..row - 1).rev() {
                                        let next = grid[j][col];
                                        match next {
                                            Some(wh2) => match wh2 {
                                                Bx => continue,
                                                Wall => continue 'main,
                                            },
                                            None => {
                                                grid[j][col] = Some(Bx);
                                                grid[row - 1][col] = None;
                                                row = row - 1;
                                                continue 'main;
                                            }
                                        }
                                    }
                                }
                            }
                            Wall => continue 'main,
                        },
                        None => row -= 1,
                    }
                }
            }
            B => {
                if row < rows {
                    match grid[row + 1][col] {
                        Some(wh) => match wh {
                            Bx => {
                                if row < rows - 1 {
                                    for j in row + 2..rows {
                                        let next = grid[j][col];
                                        match next {
                                            Some(wh2) => match wh2 {
                                                Bx => continue,
                                                Wall => continue 'main,
                                            },
                                            None => {
                                                grid[j][col] = Some(Bx);
                                                grid[row + 1][col] = None;
                                                row += 1;
                                                continue 'main;
                                            }
                                        }
                                    }
                                }
                            }
                            Wall => continue 'main,
                        },
                        None => row += 1,
                    }
                }
            }
            L => {
                if col > 0 {
                    match grid[row][col - 1] {
                        Some(wh) => match wh {
                            Bx => {
                                if col > 1 {
                                    for j in (0..col - 1).rev() {
                                        let next = grid[row][j];
                                        match next {
                                            Some(wh2) => match wh2 {
                                                Bx => continue,
                                                Wall => continue 'main,
                                            },
                                            None => {
                                                grid[row][j] = Some(Bx);
                                                grid[row][col - 1] = None;
                                                col -= 1;
                                                continue 'main;
                                            }
                                        }
                                    }
                                }
                            }
                            Wall => continue 'main,
                        },
                        None => col -= 1,
                    }
                }
            }
            R => {
                if col < cols {
                    match grid[row][col + 1] {
                        Some(wh) => match wh {
                            Bx => {
                                if col < cols - 1 {
                                    for j in col + 2..cols {
                                        let next = grid[row][j];
                                        match next {
                                            Some(wh2) => match wh2 {
                                                Bx => continue,
                                                Wall => continue 'main,
                                            },
                                            None => {
                                                grid[row][j] = Some(Bx);
                                                grid[row][col + 1] = None;
                                                col += 1;
                                                continue 'main;
                                            }
                                        }
                                    }
                                }
                            }
                            Wall => continue 'main,
                        },
                        None => col += 1,
                    }
                }
            }
        }
    }
    grid
}
