use ::std::fs::read_to_string;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}\n", part_a(&data));
    //println!("{}", part_b(&data));
}

fn part_a(data: &str) -> i64 {
    let registers = fill_registers(data);
    for (i, reg) in registers.into_iter().enumerate() {
        if (i + 22) % 40 <= 2 {
            eprintln!("{i} {reg}");
        }
    }
    let mut total: i64 = 0;
    let mut i = 19;
    while i < registers.len() {
        total += (i + 1) as i64 * registers[i];
        //eprintln!(">> {i} {}: +{} = {}", registers[i], (i + 1) as i64 * registers[i], total);
        i += 40;
    }
    total
}

fn part_b(data: &str) -> i64 {
    let registers = fill_registers(data);
    for (i, reg) in registers.into_iter().enumerate() {
        eprintln!("{i} {reg}");
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
    println!();
    0
}

fn fill_registers(data: &str) -> [i64; 240] {
    let mut cycle: usize = 0;
    let mut registers = [1i64; 240];
    for line in data.lines() {
        if line.starts_with("noop") {
            if cycle + 1 < registers.len() {
                registers[cycle + 1] = registers[cycle];
            } else {
                // eprintln!("endA {cycle}");
            }
            cycle += 1;
        } else if line.starts_with("addx") {
            let addition: i64 = line[5..].parse().expect("cannot parse addx digit");
            if cycle + 1 < registers.len() {
                registers[cycle + 1] = registers[cycle];
            } else {
                // eprintln!("endB {cycle}");
            }
            if cycle + 2 < registers.len() {
                registers[cycle + 2] = registers[cycle] + addition;
            } else {
                // eprintln!("endC {cycle}");
            }
            cycle += 2;
        } else {
            panic!("unknown line: {line}")
        }
    }
    registers
}
