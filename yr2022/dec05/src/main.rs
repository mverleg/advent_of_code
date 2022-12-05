use ::std::fs::read_to_string;
use std::collections::VecDeque;

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
    let mut stacks: Vec<VecDeque<char>> = vec![];
    let mut res = 0;
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
        let stack_nr: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        println!("{stack_nr}: {from} -> {to}")
    }
    res
}
