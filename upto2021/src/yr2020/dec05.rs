use ::std::fs::read_to_string;

pub fn dec05a() {
    let ids = all_sorted_ids();
    println!("{}", ids[ids.len() - 1]);
}

pub fn dec05b() {
    let ids = all_sorted_ids();
    let mut missing = ids[0];
    for id in ids {
        //println!("{} x {}", id, missing);
        if id != missing {
            break
        }
        missing += 1;
    }
    println!("{}", missing);
}

fn all_sorted_ids() -> Vec<u32> {
    let mut ids = read_to_string("dec05.txt").unwrap()
        .lines()
        .filter(|ln| !ln.is_empty())
        .map(|ln| decode_seat_id(ln))
        .collect::<Vec<_>>();
    ids.sort();
    ids
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
