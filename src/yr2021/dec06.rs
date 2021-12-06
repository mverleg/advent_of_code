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
    let res = run(20);
    println!("{}", res);
}

pub fn part_b() {
    // let res = run(18);
    // println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Res {
    id: u32,
    price: u32,
}

fn run(remaining: u64) -> u64 {
    get_lines("data/2021/dec06.txt")[0].split(",")
        .map(|nr| nr.parse::<u64>().unwrap())
        .map(|nr| end_count(nr, remaining, "init"))
        .sum()
}

fn end_count(day: u64, remaining: u64, msg: &str) -> u64 {
    let msg = format!("{} -> {} / {}", msg, day, remaining);
    eprintln!("{}", msg);
    if day > remaining {
        return if remaining > 0 {
            1
        } else {
            // do not count the current fish in last iteration,
            // because I am spawning them one round early.
            0
        }
    }

    let mut spawn_at = (remaining - day) % CYCLE;
    let mut sum = 1;  // current
    while spawn_at + day <= remaining {
        sum += end_count(INIT, spawn_at, &msg);
        spawn_at += CYCLE;
    }
    sum
}

#[test]
fn test1() {
    assert_eq!(end_count(5, 3, "test"), 1);
}

#[test]
fn test2() {
    assert_eq!(end_count(1, 5, "test"), 2);
}

#[test]
fn test3() {
    assert_eq!(end_count(1, 11, "test"), 4);
}

#[test]
fn test6() {
    assert_eq!(end_count(0, 1, "test"), 2);
}

#[test]
fn test7() {
    // new spawns only count next round (when current fish resets back to 6)
    assert_eq!(end_count(1, 1, "test"), 1);
}

#[test]
fn test8() {
    // new spawns only count next round (when current fish resets back to 6)
    assert_eq!(end_count(0, 0, "test"), 1);
}

#[test]
fn test9() {
    //TODO: verify answer
    assert_eq!(end_count(0, 46, "test"), 71);
}

#[test]
fn test_pzzl1() {
    assert_eq!(end_count(1, 18, "test"), 7);
}

#[test]
fn test_pzzl2() {
    assert_eq!(end_count(2, 18, "test"), 5);
}

#[test]
fn test_pzzl3() {
    assert_eq!(end_count(3, 18, "test"), 5);
}

#[test]
fn test_pzzl4() {
    assert_eq!(end_count(4, 18, "test"), 4);
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
