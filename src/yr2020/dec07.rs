use ::std::collections::HashMap;
use ::std::fs::read_to_string;
use std::collections::HashSet;

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
    let mut map = HashMap::<String, HashSet<String>>::new();
    for line in get_lines("data/2020/dec07.txt") {
        let (outer, inner) = line.split_once(" bags contain ").unwrap();
        if inner == "no other bags." {
            continue;
        }
        let inners = inner.split(", ")
            .map(|bag| {
                let parts = bag.split(" ").collect::<Vec<_>>();
                format!("{} {}", parts[1], parts[2])
            })
            .collect::<Vec<_>>();
        for inner in inners {
            if !map.contains_key(&inner) {
                map.insert(inner.clone(), HashSet::new());
            }
            map.get_mut(&inner).unwrap().insert(outer.to_owned());
        }
    }

    dbg!(find_outer(&map, "shiny gold")); //TODO @mark: TEMPORARY! REMOVE THIS!
    find_outer(&map, "shiny gold").len() as u64
}

fn find_outer(map: &HashMap<String, HashSet<String>>, color: &str) -> HashSet<String> {
    let mut set = map.get(color)
        .map(|s| s.clone())
        .unwrap_or_else(|| HashSet::new());
    for outer_col in set.clone() {
        // if set.contains(&outer_col) {
        //     continue
        // }
        set.extend(find_outer(map, &outer_col))
    }
    set
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
