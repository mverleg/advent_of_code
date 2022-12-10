use ::std::fs::read_to_string;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}\n", part_a(&data));
    println!("{}", part_b(&data));
}

fn part_a(data: &str) -> i64 {
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
            //eprintln!("{cycle} {register} +{extra_signal_strength} = {signal_strength}");
            next_recording_cycle += 40;
        }
        register += addition;
    }
    signal_strength
}

fn part_b(data: &str) -> i64 {
    let mut screen = [false; 240];
    let mut cycle: i64 = 0;
    let mut register = 1;
    for line in data.lines() {
        let mut addition: i64 = 0;
        let mut cycle_step;
        if line.starts_with("noop") {
            cycle_step = 1
        } else if line.starts_with("addx") {
            addition = line[5..].parse().expect("cannot parse addx digit");
            cycle_step = 2;
        } else {
            panic!("unknown line: {line}")
        }

        for offset in 1..=cycle_step {
            eprint!("{cycle}+{offset} ({}) {register}", cycle+offset);
            let is_sprite = cycle+offset >= register - 1 && cycle+offset <= register + 1;
            screen[(cycle + offset) as usize % screen.len()] = is_sprite;
            if is_sprite {
                eprintln!(" DRAW");
            } else {
                eprintln!(" .");
            }
        }
        cycle += cycle_step;
        register += addition;
    }
    for (i, pixel) in screen.into_iter().enumerate() {
        if i % 40 == 0 {
            println!()
        }
        if pixel {
            print!("#")
        } else {
            print!(".")
        }
    }
    println!();
    0
}
