use std::collections::HashSet;
#[derive(Debug)]
pub struct Input {
    pub start_pos: (usize, usize),
    pub grid: Vec<Vec<bool>>,
}
impl Input {
    pub fn get_path(&self) -> Vec<(usize, usize)> {
        let mut path = vec![];
        let mut pos = self.start_pos;
        let grid = &self.grid;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dir = Direction::new();
        loop {
            let d = dir.get();
            path.push(pos);
            let next_pos = (
                usize::try_from(pos.0 as i32 + d.0),
                usize::try_from(pos.1 as i32 + d.1),
            );
            match next_pos {
                (Ok(r), Ok(c)) => {
                    if r < rows && c < cols {
                        if grid[r][c] {
                            pos = (r, c);
                        } else {
                            dir.next();
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
        path
    }
    pub fn get_obstacle_loop_count(&mut self) -> usize {
        let mut obstacle_hs = HashSet::new();
        let path = self.get_path();
        for pos in path {
            if pos != self.start_pos {
                self.grid[pos.0][pos.1] = false;
                if self.is_looping() {
                    obstacle_hs.insert((pos.0, pos.1));
                }
                self.grid[pos.0][pos.1] = true;
            }
        }
        obstacle_hs.len()
    }
    fn is_looping(&self) -> bool {
        let mut pos_hs = HashSet::new();
        let mut pos = self.start_pos;
        let grid = &self.grid;
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dir = Direction::new();
        loop {
            if pos_hs.contains(&(pos, dir.get())) {
                return true;
            }
            let d = dir.get();
            pos_hs.insert((pos, d));
            let next_pos = (
                usize::try_from(pos.0 as i32 + d.0),
                usize::try_from(pos.1 as i32 + d.1),
            );
            match next_pos {
                (Ok(r), Ok(c)) => {
                    if r < rows && c < cols {
                        if grid[r][c] {
                            pos = (r, c);
                        } else {
                            dir.next();
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
        false
    }
}
#[derive(Clone, Copy)]
pub struct Direction {
    curr: usize,
}
impl Direction {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    pub fn new() -> Self {
        Direction { curr: 0 }
    }
    pub fn get(&self) -> (i32, i32) {
        Self::DIRS[self.curr]
    }
    pub fn next(&mut self) -> (i32, i32) {
        match self.curr {
            3 => self.curr = 0,
            _ => self.curr += 1,
        }
        Self::DIRS[self.curr]
    }
}

pub fn parse_inp(inp: &str) -> Input {
    let mut start_pos = (0, 0);
    let mut grid = vec![];
    for (row, line) in inp.lines().enumerate() {
        let mut r = Vec::with_capacity(line.len());
        for (col, c) in line.char_indices() {
            match c {
                '.' => r.push(true),
                '#' => r.push(false),
                '^' => {
                    start_pos = (row, col);
                    r.push(true);
                }
                _ => {}
            }
        }
        grid.push(r);
    }
    Input { start_pos, grid }
}
