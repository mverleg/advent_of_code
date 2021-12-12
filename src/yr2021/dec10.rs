use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;
use nom::combinator::map;

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run2();
    println!("{}", res);
}

#[derive(Debug)]
enum Res {
    Ok,
    Err(char),
    Incomplete(Vec<char>),
}

fn run1() -> u64 {
    get_lines("data/2021/dec10.txt").into_iter()
        .map(|line| analyze(&line))
        .flat_map(|res| match res {
            Res::Ok => None,
            Res::Err(unexpected) => Some(unexpected),
            Res::Incomplete(_) => None,
        }.into_iter())
        .map(score_error)
        .sum()
}

fn run2() -> u64 {
    get_lines("data/2021/dec10.txt").into_iter()
        .map(|line| analyze(&line))
        .flat_map(|res| match res {
            Res::Ok => None,
            Res::Err(_) => None,
            Res::Incomplete(missing) => Some(missing),
        }.into_iter())
        .map(score_missing)
        .sum()
}

fn analyze(inp: &Vec<char>) -> Res {
    let mut stack = vec![];
    for c in inp {
        match c {
            open @ ('(' | '[' | '{' | '<') => stack.push(*open),
            close @ (')' | ']' | '}' | '>') => {
                let open = match stack.pop() {
                    Some(open) => open,
                    None => panic!("too many closers, not supported"),
                };
                if find_closing(open) != *close {
                    return Res::Err(*close)
                }
            },
            _ => panic!(),
        }
    }
    if stack.is_empty() {
        Res::Ok
    } else {
        Res::Incomplete(close_stack(stack))
    }
}

fn close_stack(mut pending: Vec<char>) -> Vec<char> {
    let mut closers = vec![];
    while let Some(open) = pending.pop() {
        closers.push(find_closing(open))
    }
    closers
}

fn find_closing(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn score_error(closer: char) -> u64 {
    match closer {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn score_missing(closers: Vec<char>) -> u64 {

    unimplemented!()
}

fn get_lines(pth: &str) -> Vec<Vec<char>> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
