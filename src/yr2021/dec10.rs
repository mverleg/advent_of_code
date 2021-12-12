use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;
use nom::combinator::map;

pub fn part_a() {
    let res = run();
    println!("{}", res);
}

pub fn part_b() {
    let res = run();
    println!("{}", res);
}

#[derive(Debug)]
enum Res {
    Ok,
    Err(char),
    Incomplete,
}

fn run() -> u64 {
    get_lines("data/2021/dec10.txt").into_iter()
        .map(|line| analyze(&line))
        .flat_map(|res| match res {
            Res::Ok => None,
            Res::Err(c) => Some(c),
            Res::Incomplete => None,
        }.into_iter())
        .map(score_closer)
        .sum()
}

fn analyze(inp: &Vec<char>) -> Res {
    unimplemented!()
}

fn score_closer(closer: char) -> u64 {
    match closer {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn get_lines(pth: &str) -> Vec<Vec<char>> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
