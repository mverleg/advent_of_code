use ::std::fs::read_to_string;
use std::collections::VecDeque;

// ~23:15

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}

fn part_a(data: &str) -> String {
    run(data, false)
}

fn part_b(data: &str) -> String {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> String {
    let mut stacks: Vec<VecDeque<char>> = vec![];
    let mut rev_lines = data.lines().rev().collect::<Vec<_>>();
    // Parse stacks
    while let Some(line) = rev_lines.pop() {
        let chars = line.chars().collect::<Vec<_>>();
        if line.is_empty() {
            break
        }
        for n in 0 .. 1000usize {
            let m = 1 + 4 * n;
            if let Some(c) = chars.get(m) {
                if *c == '1' {
                    break
                }
                if *c == ' ' {
                    continue
                }
                while stacks.len() <= n {
                    stacks.push(VecDeque::new())
                }
                stacks[n].push_front(*c)
            }
        }
    }
    // Parse/do operations
    while let Some(line) = rev_lines.pop() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let amt: usize = parts[1].parse().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1usize;
        let to = parts[5].parse::<usize>().unwrap() - 1usize;
        if ! is_b {
            for _ in 0..amt {
                let val = stacks[from].pop_back().unwrap();
                stacks[to].push_back(val);
                //println!("{from} -> {to}")
            }
        } else {
            let mut moving = vec![];
            for _ in 0..amt {
                let val = stacks[from].pop_back().unwrap();
                moving.push(val);
                //println!("{from} -> {to}")
            }
            moving.reverse();
            for val in moving {
                stacks[to].push_back(val);
            }
        }
    }
    let mut result = String::new();
    for mut stack in stacks {
        if let Some(val) = stack.pop_back() {
            result.push(val)
        } else {
            eprintln!("empty stack")
        }
    }
    result
}
