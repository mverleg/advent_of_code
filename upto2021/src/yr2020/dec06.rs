use ::std::fs::read_to_string;
use ::std::collections::HashMap;
use std::collections::HashSet;

pub fn dec06a() {
    let res = run(unique_answers);
    println!("{}", res);
}

pub fn dec06b() {
    let res = run(universal_answers);
    println!("{}", res);
}

fn run(group_counter: fn(&[&str]) -> u64) -> u64 {
    read_to_string("../../data/2020/dec06.txt")
        .unwrap()
        .lines()
        .collect::<Vec<_>>()
        .split(|ln| ln.trim().is_empty())
        .map(group_counter)
        .sum()
}

fn unique_answers(group: &[&str]) -> u64 {
    let mut uniq = HashSet::new();
    group.iter()
        .flat_map(|ans| ans.chars())
        .for_each(|ans| {uniq.insert(ans);} );
    uniq.len() as u64
}

fn universal_answers(group: &[&str]) -> u64 {
    group.iter()
        .map(|ans| ans.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b)
            .cloned()
            .collect::<HashSet<char>>())
        .unwrap()
        .len() as u64
}
