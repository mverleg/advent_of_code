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
    let mut cycle: usize = 0;
    let mut registers = [0i64; 240];
    for line in data.lines() {
        if line.starts_with("noop") {
            if cycle+1 < registers.len() {
                registers[cycle + 1] = registers[cycle];
            } else {
                // eprintln!("endA {cycle}");
            }
            cycle += 1;
        } else if line.starts_with("addx") {
            let addition: i64 = line[5..].parse().expect("cannot parse addx digit");
            if cycle+1 < registers.len() {
                registers[cycle + 1] = registers[cycle];
            } else {
                // eprintln!("endB {cycle}");
            }
            if cycle+2 < registers.len() {
                registers[cycle + 2] = registers[cycle] + addition;
            } else {
                // eprintln!("endC {cycle}");
            }
            cycle += 2;
        } else {
            panic!("unknown line: {line}")
        }
        // for offset in 1..=cycle_step {
        //     eprint!("{cycle}+{offset} ({}) {register} +{addition}", cycle+offset);
        //     let is_sprite = cycle+offset >= register - 1 && cycle+offset <= register + 1;
        //     screen[(cycle + offset - 1) as usize % screen.len()] = is_sprite;
        //     if is_sprite {
        //         eprintln!(" DRAW");
        //     } else {
        //         eprintln!(" .");
        //     }
        // }
        // cycle += cycle_step;
        // register += addition;
    }
    for (i, reg) in registers.into_iter().enumerate() {
        //eprintln!("{i} {reg}");
        let row_i = (i % 40) as i64;
        if row_i == 0 {
            println!();
        }
        let is_pixel_on = row_i == reg;
        if is_pixel_on {
            print!("#");
        } else {
            print!(".");
        }
    }
    // for (i, pixel) in screen.into_iter().enumerate() {
    //     if i % 40 == 0 {
    //         println!()
    //     }
    //     if pixel {
    //         print!("#")
    //     } else {
    //         print!(".")
    //     }
    // }
    println!();
    0
}
