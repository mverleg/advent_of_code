use ::std::fs::read_to_string;
use std::collections::HashSet;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let mut res = 0;
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut seen = HashSet::new();
    for line in data.lines() {
        let (dir, amt) = line.split_once(' ').expect("cannot parse line");
        let amt = amt.parse().expect("incorrect step size");
        for _ in 0..amt {
            match dir {
                "R" => head.0 += 1,
                "U" => head.1 += 1,
                "L" => head.0 -= 1,
                "D" => head.1 -= 1,
                _ => panic!("unknown direction"),
            }
            seen.insert(tail);
        }
    }
    eprintln!("head final pos: {},{}", head.0, head.1);
    res
}
