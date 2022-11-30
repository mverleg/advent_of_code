use ::std::fs::read_to_string;
use ::std::collections::HashMap;

use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w+):\w*([0-9]+)$").unwrap();
}

pub fn dec01a() {
    let res = run(false);
    println!("{}", res);
}

pub fn dec01b() {
    let res = run(true);
    println!("{}", res);
}

fn run(slide: bool) -> u64 {
    let mut depths = read_to_string("../../data/2021/dec01.txt")
        .unwrap()
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    if slide {
        let mut slid = vec![];
        for i in 0 .. depths.len() - 2 {
            slid.push(depths[i] + depths[i + 1] + depths[i + 2]);
        }
        depths = slid;
    }
    let mut last = depths[0];
    let mut increase_cnt = 0;
    for dep in depths {
        if dep > last {
            increase_cnt += 1;
        }
        last = dep;
    }
    increase_cnt
}
