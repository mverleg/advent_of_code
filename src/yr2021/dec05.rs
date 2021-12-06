use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::sequence::tuple;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn part_a() {
    let res = run();
    println!("{}", res);
}

pub fn part_b() {
    let res = run();
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn run() -> u64 {
    // Find id with highest total price

    get_lines("data/2021/dec05.txt").into_iter()
        .map(|line| parse_line);

    unimplemented!()
}

fn parse_line(line: &str) -> Res {
    map(tuple((digit1, tag(","), digit1, tag(" -> "), digit1, tag(","), digit1)),
        |(x1, _, y1, _, x2, _, y2): (&str, &str, &str, &str, &str, &str, &str)| Res {
            x1: x1.parse::<u32>().unwrap(),
            y1: y1.parse::<u32>().unwrap(),
            x2: x2.parse::<u32>().unwrap(),
            y2: y2.parse::<u32>().unwrap()
        })(line).unwrap().1
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
