use ::std::fs::read_to_string;
use std::ops::BitXor;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s+(\w):\s+(\w+)").unwrap();
}

pub fn dec02a() {
    let valid_cnt = process_file(validate_cnt);
    println!("{}", valid_cnt);
}

pub fn dec02b() {
    let valid_cnt = process_file(validate_pos);
    println!("{}", valid_cnt);
}

fn process_file(validate: fn(&Pwd) -> bool) -> usize {
    let valid_cnt = read_to_string("dec02.txt")
        .unwrap()
        .lines()
        .filter(|ln| !ln.is_empty())
        .map(|ln| parse(ln))
        .filter(|pwd| validate_pos(pwd))
        .count();
    valid_cnt
}

#[derive(Debug)]
struct Pwd {
    min: u32,
    max: u32,
    letter: char,
    pass: String,
}

fn parse(line: &str) -> Pwd {
    let groups = RE.captures(line).unwrap();
    Pwd {
        min: groups[1].parse().unwrap(),
        max: groups[2].parse().unwrap(),
        letter: groups[3].chars().next().unwrap(),
        pass: groups[4].to_owned(),
    }
}

fn validate_cnt(pwd: &Pwd) -> bool {
    let cnt = pwd.pass.chars()
        .filter(|c| c == &pwd.letter)
        .count() as u32;
    cnt >= pwd.min && cnt <= pwd.max
}

fn validate_pos(pwd: &Pwd) -> bool {
    let passchars = pwd.pass.chars().collect::<Vec<char>>();
    let first_match = passchars[pwd.min as usize - 1] == pwd.letter;
    let second_match = passchars[pwd.max as usize - 1] == pwd.letter;
    first_match ^ second_match
}
