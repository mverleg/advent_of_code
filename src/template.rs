use ::std::fs::read_to_string;
use ::std::collections::HashMap;

use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w+):\w*([0-9]+)$").unwrap();
}

pub fn dec00a() {
    let res = run();
    println!("{}", res);
}

pub fn dec00b() {
    let res = run();
    println!("{}", res);
}

// #[derive(Debug)]
// struct Pwd {
//     min: u32,
//     max: u32,
//     letter: char,
//     pass: String,
// }

fn run() -> u64 {
    unimplemented!();
}
    // let lines = read_to_string("data/2021/dec00.txt")
    //     .unwrap()
//         .lines()
//         .filter(|ln| !ln.trim().is_empty())
//         .map(|line| {
//             let groups = RE.captures(line).unwrap();
//             // Pwd {
//             //     min: groups[1].parse().unwrap(),
//             //     max: groups[2].parse().unwrap(),
//             //     letter: groups[3].chars().next().unwrap(),
//             //     pass: groups[4].to_owned(),
//             // }
//             (0u32, 0u32)
//         })
//         .collect::<Vec<_>>();
//
//     let mut counts = HashMap::new();
//     // for line in lines {
//     //     *counts.entry(line[0]).or_insert(0) += 1;
//     // }
//     let mut top = counts.into_iter()
//         .map(|(name, count)| (count, name))
//         .collect::<Vec<_>>();
//     top.sort_by_key(|(count, name)| -count);
//     unimplemented!()
// }
