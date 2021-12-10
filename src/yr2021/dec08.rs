use ::std::collections::HashMap;
use ::std::fs::read_to_string;
use ::std::collections::HashSet;

use ::itertools::Itertools;

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run2();
    println!("{}", res);
}

fn run1() -> u64 {
    parse_input().iter()
        .map(|(_, outp)| outp)
        .map(|outp| outp.iter()
            .map(|word| word.len())
            .filter(|len| len <= &4 || len == &7)
            .count())
        .sum::<usize>() as u64
}

fn run2() -> u64 {
    parse_input().iter()
        .map(|(_, outp)| outp)
        .map(|outp| outp.iter()
            .map(|word| word.len())
            .filter(|len| len <= &4 || len == &7)
            .count())
        .sum::<usize>() as u64
}

fn parse_input() -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    let inps = get_lines("data/2021/dec08.txt").iter()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(ptrns, outp)| (space_sep_ints(ptrns), space_sep_ints(outp)))
        .collect::<Vec<_>>();
    inps
}

fn space_sep_ints(txt: &str) -> Vec<HashSet<char>> {
    txt.split(" ")
        .map(|word| word.chars().collect::<HashSet<_>>())
        .collect()
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
