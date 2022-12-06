use ::std::fs::read_to_string;
use std::collections::HashSet;

// ~9 min

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}


fn part_a(data: &str) -> usize {
    run(data, 4)
}

fn part_b(data: &str) -> usize {
    run(data, 14)
}

fn run(data: &str, marker_len: usize) -> usize {
    let chars = data.chars().collect::<Vec<_>>();
    for i in marker_len-1..chars.len() {
        let mut last = HashSet::new();
        for j in 0 .. marker_len {
            //eprintln!("{i} {j}");
            last.insert(chars[i - j]);
        }
        if last.len() == marker_len {
            return i + 1;
        }
    }
    panic!("no marker")
}
