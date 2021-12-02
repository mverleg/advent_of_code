use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn dec02a() {
    let res = run();
    println!("{}", res);
}

pub fn dec02b() {
    let res = run();
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    id: u32,
    price: u32,
}

fn run() -> u64 {
    let content = read_to_string("data/2021/dec02.txt").unwrap();
    let lines = content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .collect::<Vec<_>>();
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
            },
            "up" => {
                aim -= amt
            },
            "down" => {
                aim += amt
            },
            _ => unreachable!(),
        }
    }
    hor * depth
}
