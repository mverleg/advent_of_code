use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;

pub fn dec02a() {
    let lines = get_lines();
    let mut hor = 0u64;
    let mut depth = 0u64;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<_>>();
        let (dir, amt) = (parts[0], parts[1].parse::<u64>().unwrap());
        match dir {
            "forward" => {
                hor += amt
            }
            "up" => {
                depth -= amt
            }
            "down" => {
                depth += amt
            }
            _ => unreachable!(),
        }
    }
    let res = hor * depth;
    println!("{}", res);
}

pub fn dec02b() {
    let lines = get_lines();
    let mut hor = 0u64;
    let mut depth = 0u64;
    let mut aim = 0u64;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<_>>();
        let (dir, amt) = (parts[0], parts[1].parse::<u64>().unwrap());
        match dir {
            "forward" => {
                hor += amt;
                depth += aim * amt;
            }
            "up" => {
                aim -= amt
            }
            "down" => {
                aim += amt
            }
            _ => unreachable!(),
        }
    }
    let res = hor * depth;
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    id: u32,
    price: u32,
}

fn get_lines() -> Vec<String> {
    let content = read_to_string("../../data/2021/dec02.txt").unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}


