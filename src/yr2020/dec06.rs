use ::std::fs::read_to_string;
use ::std::collections::HashMap;
use std::collections::HashSet;

pub fn dec06a() {
    let res = run();
    println!("{}", res);
}

pub fn dec06b() {
    let res = run();
    println!("{}", res);
}

fn run() -> u64 {
    read_to_string("data/2020/dec06.txt")
        .unwrap()
        .lines()
        .collect::<Vec<_>>()
        .split(|ln| ln.trim().is_empty())
        .map(|grp| {
            grp.iter()
                .map(|ans| ans.chars().collect::<HashSet<char>>())
                .reduce(|a, b| a.intersection(&b)
                    .cloned()
                    .collect::<HashSet<char>>() )
                .unwrap()
                .len() as u64
        })
        .sum()
}
