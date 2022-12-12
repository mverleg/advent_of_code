use ::std::fs::read_to_string;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("A: {}", part_a(&data));
    println!("B: {}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let mut res = 0;
    for line in data.lines() {
        todo!()
    }
    res
}

fn parse(data: &str) -> Vec<Vec<u8>> {
    let mut grid = vec![];
    for line in data.lines() {
        let mut row = vec![];
        for chr in line.chars() {
            row.push(chr.to_digit(36) - 10);
        }
        grid.push(row);
    }
    grid
}