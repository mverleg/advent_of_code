use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

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

fn run() -> i64 {
    let poss = get_lines("data/2021/dec07.txt")
        [0].split(",")
        //.inspect(|nr| eprintln!(">> {}", nr))
        .map(|nr| nr.parse::<i64>().unwrap())
        .sorted()
        .collect::<Vec<_>>();

    //let target = ((poss.iter().sum::<i64>() as f64) / (poss.len() as f64)).round() as i64;
    let target = poss[poss.len() / 2];
    // dbg!(target);

    let cost = poss.iter()
        .map(|nr| (nr - target).abs())
        .sum::<i64>();
    // dbg!(cost);

    cost
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
