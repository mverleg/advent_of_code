use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn dec00a() {
    let res = run();
    println!("{}", res);
}

pub fn dec00b() {
    let res = run();
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    id: u32,
    price: u32,
}

fn run() -> u64 {
    // Find id with highest total price
    read_to_string("data/2021/dec00.txt")
        .unwrap()
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .map(|line| {
            let groups = RE.captures(&line).unwrap();
            Res {
                id: groups[1].parse().unwrap(),
                price: groups[2].parse().unwrap(),
            }
        })
        .into_group_map_by(|res| res.id)
        .into_iter()
        .map(|(k, v)| Res { id: k, price: v.iter().map(|res| res.price).sum() })
        .inspect(|res| println!("item {} total price {}", res.id, res.price))
        .sorted_by_key(|res| res.price)
        .rev()
        .find(|res| true)
        .unwrap()
        .id as u64
}
