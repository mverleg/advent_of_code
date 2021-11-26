use ::std::fs::read_to_string;
use std::collections::HashSet;

pub fn dec04a() {
    do_batch()
}

fn do_batch() {
    let mut current = String::with_capacity(1024);
    for line in read_to_string("dec04ex.txt").unwrap().lines() {
        if line.is_empty() {
            do_entry(&current);
            current.clear();
            continue
        }
        current.push_str(" ");
        current.push_str(line);
    }
}

fn do_entry(inp: &str) {
    let keys = inp.split(' ')
        .map(|s| s.chars().take(3).collect::<String>())
        .filter(|s| !s.is_empty())
        .collect::<HashSet<_>>();
    println!("keys = {:?}", &keys);
}
