use ::std::fs::read_to_string;

// 11:07

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    eprintln!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    data.lines()
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            "" => 0,
            _ => todo!(),
        }).sum()
}

fn part_b(data: &str) -> usize {
    data.lines()
        .map(|line| match line {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            "" => 0,
            _ => todo!(),
        }).sum()
}
