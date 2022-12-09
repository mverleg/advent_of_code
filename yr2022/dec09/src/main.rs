use ::std::fs::read_to_string;
use std::collections::HashSet;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    run(data, 2)
}

fn part_b(data: &str) -> usize {
    run(data, 10)
}

fn run(data: &str, rope_len: usize) -> usize {
    let mut chain = vec![(0i32, 0i32); rope_len];
    let mut seen = HashSet::new();
    for line in data.lines() {
        let (dir, amt) = line.split_once(' ').expect("cannot parse line");
        let amt = amt.parse().expect("incorrect step size");
        for _ in 0..amt {
            let head = &mut chain[0];
            match dir {
                "R" => head.0 += 1,
                "U" => head.1 += 1,
                "L" => head.0 -= 1,
                "D" => head.1 -= 1,
                _ => panic!("unknown direction"),
            }
            for knot in 0..chain.len() - 1 {
                let head = chain[knot];
                let tail = &mut chain[knot + 1];
                if head.0 > tail.0 + 1 {
                    *tail = (tail.0 + 1, tail.1 + (head.1 - tail.1).signum());
                } else if head.0 < tail.0 - 1 {
                    *tail = (tail.0 - 1, tail.1 + (head.1 - tail.1).signum());
                } else if head.1 > tail.1 + 1 {
                    *tail = (tail.0 + (head.0 - tail.0).signum(), tail.1 + 1);
                } else if head.1 < tail.1 - 1 {
                    *tail = (tail.0 + (head.0 - tail.0).signum(), tail.1 - 1);
                } else {
                    // do not move
                }
            }
            eprintln!("head pos: {},{}  tail pos: {},{} after {dir} {amt}",
                      chain[0].0, chain[0].1, chain[rope_len - 1].0, chain[rope_len - 1].1);
            seen.insert(chain[rope_len - 1]);
        }
    }
    eprintln!("final head pos: {},{}  tail pos: {},{}",
              chain[0].0, chain[0].1, chain[rope_len - 1].0, chain[rope_len - 1].1);
    seen.len()
}
