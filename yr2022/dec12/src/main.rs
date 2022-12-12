use ::std::fs::read_to_string;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("A: {}", part_a(&data));
    println!("B: {}", part_b(&data));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let (start, end, grid) = parse(data);
    dbg!(&grid);
    todo!();
}

fn parse(data: &str) -> (Pos, Pos, Vec<Vec<u8>>) {
    let mut start = Pos { x: usize::MAX, y: usize::MAX };
    let mut end = Pos { x: usize::MAX, y: usize::MAX };
    let mut grid = vec![];
    for (x, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (y, chr) in line.chars().enumerate() {
            let val = if chr == 'S' {
                start = Pos { x, y };
                0
            } else if chr == 'E' {
                end = Pos { x, y };
                25
            } else {
                chr.to_digit(36).expect("not a b36 digit") - 10
            };
            assert!(val >=0 && val < 26);
            row.push(val as u8);
        }
        grid.push(row);
    }
    assert!(start.x < usize::MAX);
    assert!(start.y < usize::MAX);
    assert!(end.x < usize::MAX);
    assert!(end.y < usize::MAX);
    (start, end, grid)
}