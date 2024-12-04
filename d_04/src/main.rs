#![allow(dead_code, unused)]

use std::fs;
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_p1(&inp);
    let p2 = calc_p2(&inp);
    println!("{}", p1);
    println!("{}", p2);
}
fn calc_p1(inp: &str) -> u64 {
    let mut res = 0;
    let mut table = parse_to_table(inp);
    for (j, row) in table.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            let count = count_xmas(&table, (i, j));
            res += count;
        }
    }
    res
}
fn calc_p2(inp: &str) -> u64 {
    let mut res = 0;
    let mut table = parse_to_table(inp);
    for (j, row) in table.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            let is_mas = check_mas(&table, (i, j));
            if is_mas {
                res += 1;
            }
        }
    }
    res
}
fn check_mas(t: &[Vec<char>], loc: (usize, usize)) -> bool {
    let n_col = t.len();
    let n_rows = t[0].len();
    let (y, x) = loc;
    //Check if in bound
    if t[y][x] == 'A'
        && y > 0
        && y < n_rows - 1
        && x > 0
        && x < n_col - 1
        && ((t[y - 1][x - 1] == 'M' && t[y + 1][x + 1] == 'S')
            || (t[y + 1][x + 1] == 'M' && t[y - 1][x - 1] == 'S'))
        && ((t[y - 1][x + 1] == 'M' && t[y + 1][x - 1] == 'S')
            || (t[y + 1][x - 1] == 'M' && t[y - 1][x + 1] == 'S'))
    {
        return true;
    }
    false
}
fn count_xmas(t: &[Vec<char>], loc: (usize, usize)) -> u64 {
    let mut res = 0;
    let n_col = t.len();
    let n_rows = t[0].len();
    let (y, x) = loc;
    if t[y][x] == 'X' {
        //RIGHT
        if n_rows - x >= 4 && t[y][x + 1] == 'M' && t[y][x + 2] == 'A' && t[y][x + 3] == 'S' {
            res += 1;
        }
        //LEFT
        if x >= 3 && t[y][x - 1] == 'M' && t[y][x - 2] == 'A' && t[y][x - 3] == 'S' {
            res += 1;
        }
        //TOP
        if y >= 3 && t[y - 1][x] == 'M' && t[y - 2][x] == 'A' && t[y - 3][x] == 'S' {
            res += 1;
        }
        //BOTTOM
        if n_col - y >= 4 && t[y + 1][x] == 'M' && t[y + 2][x] == 'A' && t[y + 3][x] == 'S' {
            res += 1;
        }
        //RIGHT-BOTTOM
        if n_rows - x >= 4
            && n_col - y >= 4
            && t[y + 1][x + 1] == 'M'
            && t[y + 2][x + 2] == 'A'
            && t[y + 3][x + 3] == 'S'
        {
            res += 1;
        }
        //RIGHT-TOP
        if n_rows - x >= 4
            && y >= 3
            && t[y - 1][x + 1] == 'M'
            && t[y - 2][x + 2] == 'A'
            && t[y - 3][x + 3] == 'S'
        {
            res += 1;
        }
        //LEFT-BOTTOM
        if x >= 3
            && n_col - y >= 4
            && t[y + 1][x - 1] == 'M'
            && t[y + 2][x - 2] == 'A'
            && t[y + 3][x - 3] == 'S'
        {
            res += 1;
        }
        //LEFT-TOP
        if y >= 3
            && x >= 3
            && t[y - 1][x - 1] == 'M'
            && t[y - 2][x - 2] == 'A'
            && t[y - 3][x - 3] == 'S'
        {
            res += 1;
        }
    }
    res
}
fn is_start_of_word(s: &str) -> bool {
    true
}
fn parse_to_table(inp: &str) -> Vec<Vec<char>> {
    let mut n_rows = inp.lines().next().unwrap().chars().count();
    let mut n_lines = inp.lines().count();
    let mut table: Vec<Vec<char>> = Vec::with_capacity(n_lines);
    let mut i = 0;
    while i < n_lines {
        table.push(Vec::with_capacity(n_rows));
        i += 1;
    }
    for (i, l) in inp.lines().enumerate() {
        for c in l.chars() {
            table[i].push(c);
        }
    }
    table
}
