use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run2(true) * run2(false);
    println!("{}", res);
}

fn run1() -> u64 {
    let lines = get_lines("data/2021/dec03.txt");
    let line_cnt = lines.len();
    let digit_cnt = lines[0].len();
    let mut one_cnts = vec![0u64; digit_cnt];
    for line in lines {
        let chrs = line.chars().collect::<Vec<_>>();
        for c in 0 .. digit_cnt {
            if chrs[c] == '1' {
                one_cnts[c] += 1
            }
        }
    }
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
    }
    let gam = u64::from_str_radix(&gamma, 2).unwrap();
    let eps = u64::from_str_radix(&epsilon, 2).unwrap();
    gam * eps
}

fn run2(keep_most_common: bool) -> u64 {
    let mut lines = get_lines("data/2021/dec03.txt");
    let digit_cnt = lines[0].len();
    let mut one_cnts = vec![0u64; digit_cnt];
    for c in 0 .. digit_cnt {
        let mut one_cnt = 0;
        for line in &lines {
            let chrs = line.chars().collect::<Vec<_>>();
            if chrs[c] == '1' {
                one_cnt += 1
            }
        }
        let is_one_most_common = 2 * one_cnt >= (lines.len() as u64);
        lines = lines.iter()
            .filter(|line| {
                let chrs = line.chars().collect::<Vec<_>>();
                let is_currently_one = chrs[c] == '1';
                match keep_most_common {
                    true => is_one_most_common == is_currently_one,
                    false => is_one_most_common == !is_currently_one,
                }
            })
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        if lines.len() == 1 {
            return u64::from_str_radix(&lines[0], 2).unwrap();
        }
        assert!(!lines.is_empty());
    }
    unimplemented!()
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
