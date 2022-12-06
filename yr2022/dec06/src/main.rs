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
    let chars = data.chars().collect::<Vec<_>>();
    for i in 3..chars.len() {
        let mut last4 = HashSet::new();
        last4.insert(chars[i - 3]);
        last4.insert(chars[i - 2]);
        last4.insert(chars[i - 1]);
        last4.insert(chars[i]);
        if last4.len() == 4 {
            return i + 1;
        }
    }
    todo!()
}
