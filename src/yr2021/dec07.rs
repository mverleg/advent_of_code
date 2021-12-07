use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;

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

    let target = poss[poss.len() / 2];

    let cost = poss.iter()
        .map(|nr| (nr - target).abs())
        .sum::<i64>();

    cost
}

fn run2() -> i64 {
    // assumes no local minima, which I think is true
    let poss = load_nrs();

    let min_cost = (poss[0] .. poss[poss.len() - 1])
        // .inspect(|c| eprintln!("> {}", c))
        .map(|target| cost_new(&poss, target))
        // .inspect(|c| eprintln!("  = {}", c))
        .min()
        .unwrap();

    min_cost
}

fn cost_new(poss: &[i64], mut target: i64) -> i64 {
    poss.iter()
        // .inspect(|nr| eprint!("    ({} - {}) * (1 + {} - {}) / 2 -> ", nr, target, nr, target))
        .map(|nr| {
            let dif = (nr - target).abs();
            dif * (dif + 1) / 2
        })
        // .inspect(|c| eprintln!("{}", c))
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
