use ::std::fs::read_to_string;

pub fn dec05a() {
    let max_seat_id = read_to_string("dec05.txt").unwrap()
        .lines()
        .filter(|ln| !ln.is_empty())
        .map(|ln| decode_seat_id(ln))
        .max().unwrap();
    println!("{}", max_seat_id);
}

fn decode_seat_id(line: &str) -> u32 {
    let row: u32 = line.chars()
        .rev()
        .skip(3)
        .enumerate()
        .map(|(i, ch)| match ch {
            'B' => 2u32.pow(i as u32),
            'F' => 0,
            _ => panic!(),
        }).sum();
    assert!(row < 128);

    let col: u32 = line.chars()
        .rev()
        .take(3)
        .enumerate()
        .map(|(i, ch)| match ch {
            'R' => 2u32.pow(i as u32),
            'L' => 0,
            _ => panic!(),
        }).sum();
    assert!(col < 8);

    row * 8 + col
}
