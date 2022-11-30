use ::std::fs::read_to_string;

fn main() {
    let data = read_to_string("data.txt").unwrap();
    eprintln!("{}", part_a(&data));
    //eprintln!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    run(data)
}

fn part_b(data: &str) -> usize {
    run(data)
}

fn run(data: &str) -> usize {
    todo!()
}
