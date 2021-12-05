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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    id: u32,
    price: u32,
}

fn run() -> u64 {
    let lines = get_lines("data/2021/dec03.txt");
    let digit_cnt = lines[0].len();
    let mut one_cnts = vec![0u64; digit_cnt];
    let mut line_cnt = 0;
    for line in lines {
        line_cnt += 1;
        let chrs = line.chars().collect::<Vec<_>>();
        for c in 0 .. digit_cnt {
            if chrs[c] == '1' {
                one_cnts[c] += 1
            }
        }
    }
    dbg!(&one_cnts);
    let mut gamma = String::with_capacity(digit_cnt);
    let mut epsilon = String::with_capacity(digit_cnt);
    for ones in one_cnts {
        if 2 * ones > (line_cnt as u64) {
            gamma.push('1');
            epsilon.push('0');
        } else {
            epsilon.push('1');
            gamma.push('0');
        }
        println!("{} {} {:?} {:?}", ones, line_cnt, gamma, epsilon);
    }
    dbg!(&gamma, &epsilon);
    let gam = u64::from_str_radix(&gamma, 2).unwrap();
    let eps = u64::from_str_radix(&epsilon, 2).unwrap();
    dbg!(gam, eps);
    gam * eps
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
