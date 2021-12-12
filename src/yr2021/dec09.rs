use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

pub fn part_a() {
    let res = run();
    println!("{}", res);
}

pub fn part_b() {
    let res = run();
    println!("{}", res);
}

fn run() -> u64 {
    let grid = get_grid("data/2021/dec10.txt");

}

fn get_grid(pth: &str) -> Vec<Vec<u8>> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.chars()
            .map(|c| (c - '0') as u8)
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
