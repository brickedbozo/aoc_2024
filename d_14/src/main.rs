#![allow(dead_code, unused, clippy::all)]

use std::fs;
#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    vx: i64,
    vy: i64,
}
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_p1(&inp);
    println!("part1: {p1}");
}
fn calc_p1(inp: &str) -> usize {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let robots = parse_input(inp);
    for rob in robots {
        let (x, y) = get_robot_pos(&rob, 100);
        if x < 50 && y < 51 {
            q1 += 1;
        } else if x < 50 && y > 51 {
            q2 += 1;
        } else if x > 50 && y < 51 {
            q3 += 1;
        } else if x > 50 && y > 51 {
            q4 += 1;
        }
    }
    q1 * q2 * q3 * q4
}
fn get_robot_pos(rob: &Robot, sec: usize) -> (usize, usize) {
    let mut res = (0, 0);
    let width = 101;
    let height = 103;
    if rob.vx < 0 {
        let a = rob.vx.abs() as usize * sec;
        let a = a % width;
        res.0 = (width - a + rob.x) % width;
    } else {
        let a = rob.vx.abs() as usize * sec;
        let a = a % width;
        res.0 = (rob.x + a) % width;
    }
    if rob.vy < 0 {
        let a = rob.vy.abs() as usize * sec;
        let a = a % height;
        res.1 = (height - a + rob.y) % height;
    } else {
        let a = rob.vy.abs() as usize * sec;
        let a = a % height;
        res.1 = (rob.y + a) % height;
    }
    res
}
fn parse_input(inp: &str) -> Vec<Robot> {
    let mut res = vec![];
    inp.lines().for_each(|l| {
        let rv = l
            .split_terminator(' ')
            .map(|e| &e[2..])
            .map(|e| {
                e.split_terminator(',')
                    .map(|n| i64::from_str_radix(n, 10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let robot = Robot {
            x: rv[0][0] as usize,
            y: rv[0][1] as usize,
            vx: rv[1][0],
            vy: rv[1][1],
        };
        res.push(robot);
    });

    res
}
