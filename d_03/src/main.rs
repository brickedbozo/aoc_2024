#![allow(dead_code, unused)]
use std::fs;
fn main() {
    let mut instruction = true;
    let mut res = 0;
    let f = fs::read_to_string("i2").unwrap();
    f.replace("\n", "");
    let fl = f.len();
    let mut i = 0;
    'main: loop {
        if i == fl - 1 {
            break;
        }
        if i + 4 < fl {
            let phrase = &f[i..i + 4];
            if phrase == "do()" {
                instruction = true;
                i += 4;
                continue 'main;
            }
        }
        if i + 7 < fl {
            let phrase = &f[i..i + 7];
            if phrase == "don't()" {
                instruction = false;
                i += 7;
                continue 'main;
            }
        }
        if i + 4 < fl {
            let phrase = &f[i..i + 4];
            if phrase == "mul(" {
                i += 4;
                let mut numbers = String::new();
                loop {
                    if i < fl {
                        let c = f.chars().nth(i).unwrap();
                        if c.is_ascii_digit() {
                            numbers.push(c);
                            i += 1;
                            continue;
                        }
                        if c == ',' {
                            i += 1;
                            break;
                        }
                        continue 'main;
                    }
                }
                let n1 = parse_str_to_number(&numbers);
                numbers.clear();
                loop {
                    if i < fl {
                        let c = f.chars().nth(i).unwrap();
                        if c.is_ascii_digit() {
                            numbers.push(c);
                            i += 1;
                            continue;
                        }
                        if c == ')' {
                            break;
                        }
                        continue 'main;
                    }
                }
                let n2 = parse_str_to_number(&numbers);
                if instruction {
                    res += n1 * n2;
                }
                i += 1;
                continue 'main;
            }
            i += 1;
            continue 'main;
        }
        break;
    }

    println!("{:?}", res);
}
fn parse_str_to_number(s: &str) -> u64 {
    let mut res = 0;
    let mut div = 1;
    for c in s.chars().rev() {
        let n = (c as u8) - b'0';
        res += n as u64 * div;
        div *= 10;
    }
    res
}
