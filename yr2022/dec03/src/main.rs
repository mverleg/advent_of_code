use ::std::fs::read_to_string;

// 6:44

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    eprintln!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    todo!()
}
