use ::std::fs::read_to_string;

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
    let mut signal_strength = 0;
    let mut cycle: usize = 0;
    let mut register = 1;
    for line in data.lines() {
        if line.starts_with("noop") {
            cycle += 1
        } else if line.starts_with("addx") {
            let addition: i64 = line[5..].parse().expect("cannot parse addx digit");
            cycle += 2;
            register += addition;
        } else {
            panic!("unknown line: {line}")
        }
        eprintln!("{cycle} {register}");
    }
    signal_strength
}
