use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

const INIT: u64 = 9;
const CYCLE: u64 = 7;

pub fn part_a() {
    let res = run(18);
    println!("{}", res);
}

pub fn part_b() {
    let res = run(2);
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
        return 1;
    }
    let mut spawn_at = remaining - day;
    let mut sum = 1;  // self
    //eprintln!("i={} day={} rem={} sum={}", i, day, remaining, sum);
    loop {
        eprintln!("at day {} with {} rem, will spawn at {} (a.o.)", day, remaining, spawn_at);
        //eprintln!("spawn: {} / {}", i, remaining);
        sum += end_count(INIT, spawn_at);
        if spawn_at > CYCLE {
            spawn_at -= CYCLE;
        } else {
            break;
        }
    }
    sum
}

#[test]
fn test1() {
    assert_eq!(end_count(5, 3), 1);
}

#[test]
fn test2() {
    assert_eq!(end_count(1, 5), 2);
}

#[test]
fn test3() {
    assert_eq!(end_count(1, 11), 4);
}

#[test]
fn test4() {
    assert_eq!(end_count(3, 18), 7);
}

#[test]
fn test5() {
    assert_eq!(end_count(0, 1), 2);
}

#[test]
fn test6() {
    assert_eq!(end_count(1, 1), 1);
}

#[test]
fn test7() {
    assert_eq!(end_count(0, 0), 1);
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
