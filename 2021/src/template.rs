use ::std::fs::read_to_string;
use std::collections::HashMap;

use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[0-9]+$").unwrap();
}

pub fn dec00a() {
    let res = run();
    println!("{}", res);
}

pub fn dec00b() {
    let res = run();
    println!("{}", res);
}

fn run() -> u64 {
    let lines = read_to_string("../dec01.txt")
        .unwrap()
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut counts = HashMap::new();
    for line in lines {
        *counts.entry(line).or_insert(0) += 1;
    }
    let mut top = counts.into_iter()
        .map(|(name, count)| (count, name))
        .collect::<Vec<_>>();
    top.sort_by_key(|(count, name)| -count);
    unimplemented!()
}
