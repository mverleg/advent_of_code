#![feature(exclusive_range_pattern)]

use ::std::fs::read_to_string;
use std::collections::HashSet;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    eprintln!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let mut prio = 0;
    for line in data.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let left = left.chars().collect::<HashSet<char>>();
        let right = right.chars().collect::<HashSet<char>>();
        for item in left {
            if right.contains(&item) {
                let p = match item {
                    'a'..='z' => to_nr(item) - to_nr('a') + 1,
                    'A'..='Z' => to_nr(item) - to_nr('A') + 26 + 1,
                    l => todo!("invalid: {l}"),
                };
                //eprintln!("{p}");
                prio += p
            }
        }
    }
    prio
}

fn to_nr(c: char) -> usize {
    <char as Into<u32>>::into(c) as usize
}