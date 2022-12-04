use ::std::fs::read_to_string;

// 9:24

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}

#[derive(Debug)]
struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn parse(text: &str) -> Self {
        let (start, end) = text.split_once("-").unwrap();
        Section {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn contains(&self, other: &Section) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Section) -> bool {
        other.start >= self.start && other.start <= self.end
    }
}

fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let mut count = 0;
    for line in data.lines() {
        let (left, right) = line.split_once(",").unwrap();
        let left = Section::parse(left);
        let right = Section::parse(right);
        if is_b {
            if left.overlaps(&right) || right.overlaps(&left) {
                count += 1;
            }
        } else {
            if left.contains(&right) || right.contains(&left) {
                count += 1;
            }
        }
    }
    count
}
