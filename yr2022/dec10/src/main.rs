use ::std::fs::read_to_string;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}

fn part_a(data: &str) -> i64 {
    run(data, false)
}

fn part_b(data: &str) -> i64 {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> i64 {
    let mut signal_strength = 0;
    let mut cycle: i64 = 0;
    let mut register = 1;
    let mut next_recording_cycle = 20;
    for line in data.lines() {
        let mut addition: i64 = 0;
        if line.starts_with("noop") {
            cycle += 1
        } else if line.starts_with("addx") {
            addition = line[5..].parse().expect("cannot parse addx digit");
            cycle += 2;
        } else {
            panic!("unknown line: {line}")
        }
        let extra_cycles = cycle - next_recording_cycle;
        if extra_cycles >= 0 {
            let extra_signal_strength = (cycle - extra_cycles) * register;
            signal_strength += extra_signal_strength;
            eprintln!("{cycle} {register} +{extra_signal_strength} = {signal_strength}");
            next_recording_cycle += 40;
        } else {
            eprintln!("{cycle} {register}");
        }
        register += addition;
    }
    signal_strength
}
