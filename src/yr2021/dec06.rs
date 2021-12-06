use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

const INIT: u64 = 6;
const CYCLE: u64 = 8;

pub fn part_a() {
    let res = run(18);
    println!("{}", res);
}

pub fn part_b() {
    let res = run(18);
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Res {
    id: u32,
    price: u32,
}

fn run(remaining: u64) -> u64 {
    get_lines("data/2021/dec06.txt")[0].split(",")
        .map(|nr| nr.parse::<u64>().unwrap())
        .map(|nr| end_count(nr, remaining))
        .sum()

}

fn end_count(day: u64, remaining: u64) -> u64 {
    if day > remaining {
        return 1
    }
    let mut i = remaining - day;
    let mut sum = 1;  // self
    eprintln!("i={} day={} rem={} sum={}", i, day, remaining, sum);
    loop {
        eprintln!("spawn: {} / {}", i, remaining);
        sum += end_count(CYCLE, i);
        if i > INIT {
            i -= INIT;
        } else {
            break
        }
    }
    sum
}

#[test]
fn end_count_test() {
    assert_eq!(end_count(5, 3), 1);
    assert_eq!(end_count(1, 5), 2);
    assert_eq!(end_count(1, 11), 4);
    assert_eq!(end_count(3, 18), 7);
}


fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
