use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn part_a() {
    let res = run(false);
    println!("{}", res);
}

pub fn part_b() {
    let res = run(true);
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Cave {
    Start,
    End,
    Big(usize),
    Small(usize),
}

fn run(allow_one_duplicate: bool) -> u64 {
    let deps = parse_deps();
    let small_caves_seen = HashSet::new();
    count_paths(&deps, small_caves_seen, allow_one_duplicate, Cave::Start)
}

fn count_paths(deps: &HashMap<Cave, Vec<Cave>>, mut seen: HashSet<Cave>, mut duplicate_remaining: bool, cave: Cave) -> u64 {
    match cave {
        Cave::Start => {
            if seen.contains(&cave) {
                return 0
            } else {
                seen.insert(cave);
            }
        },
        Cave::End => return 1,
        Cave::Big(_) => (),
        Cave::Small(_) => {
            if seen.contains(&cave) {
                if duplicate_remaining {
                    duplicate_remaining = false;
                } else {
                    return 0
                }
            } else {
                seen.insert(cave);
            }
        },
    }
    deps[&cave].iter()
        //.inspect(|dep| eprintln!("delegate to {:?}", dep))
        .map(|dep| count_paths(deps, seen.clone(), duplicate_remaining, dep.clone()))
        .sum()
}

fn parse_deps() -> HashMap<Cave, Vec<Cave>> {
    let mut cave_ids = HashMap::new();
    let mut deps = HashMap::new();
    for line in get_lines("data/2021/dec12.txt") {
        let (left, right) = line.split_once("-").unwrap();
        let left = map_cave(&mut cave_ids, left);
        let right = map_cave(&mut cave_ids, right);
        deps.entry(left).or_insert_with(|| vec![]).push(right);
        deps.entry(right).or_insert_with(|| vec![]).push(left);
    }
    deps
}

fn map_cave(cave_ids: &mut HashMap<String, usize>, cave_name: &str) -> Cave {
    let next_id = cave_ids.len();
    let id = *cave_ids.entry(cave_name.to_owned()).or_insert(next_id);
    match cave_name {
        "start" => Cave::Start,
        "end" => Cave::End,
        general => {
            if general.to_lowercase() == general {
                Cave::Small(id)
            } else {
                Cave::Big(id)
            }
        },
    }
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
