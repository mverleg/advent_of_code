use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run2();
    println!("{}", res);
}

fn run1() -> i64 {
    let poss = load_nrs();

    //let target = ((poss.iter().sum::<i64>() as f64) / (poss.len() as f64)).round() as i64;
    let target = poss[poss.len() / 2];
    // dbg!(target);

    let cost = poss.iter()
        .map(|nr| (nr - target).abs())
        .sum::<i64>();
    // dbg!(cost);

    cost
}

fn run2() -> i64 {
    // assumes no local minima, which I think is true
    let poss = load_nrs();

    let mut target = poss[poss.len() / 2];

    let min_cost = (poss[0] .. poss[poss.len() - 1])
        .min_by_key(|p| cost_new(&poss, target))
        .unwrap();

    min_cost
}

fn cost_new(poss: &[i64], mut target: i64) -> i64 {
    poss.iter()
        .map(|nr| (nr - target) * (1 + nr - target) / 2)
        .sum::<i64>()
}

fn load_nrs() -> Vec<i64> {
    let poss = get_lines("data/2021/dec07.txt")
        [0].split(",")
        //.inspect(|nr| eprintln!(">> {}", nr))
        .map(|nr| nr.parse::<i64>().unwrap())
        .sorted()
        .collect::<Vec<_>>();
    poss
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
