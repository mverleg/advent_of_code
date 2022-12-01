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

fn run(data: &str, top3: bool) -> usize {
    let mut elves: Vec<usize> = vec![];
    let mut current = vec![];
    for line in data.lines() {
        if line.is_empty() && !current.is_empty() {
            elves.push(current.iter().sum());
            current.clear()
        } else {
            current.push(line.parse::<usize>().unwrap());
        }
    }
    elves.sort();
    if top3 {
        elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap()
    } else {
        elves.pop().unwrap()
    }
}
