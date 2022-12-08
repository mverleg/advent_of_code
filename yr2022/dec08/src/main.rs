use ::std::fs::read_to_string;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let mut grid = parse_grid(data);
    let mut vis_cnt = 0;
    for row in grid.iter_mut() {
        let mut highest = 0;
        for col in row.iter_mut() {
            if *col > highest {
                highest = *col;
                vis_cnt += 1;
                *col = 0;
            }
        }
    }
    eprintln!("vis_cnt={vis_cnt}");
    todo!()
}

fn parse_grid(data: &str) -> Vec<Vec<usize>> {
    let mut grid = vec![];
    for line in data.lines() {
        let mut row = vec![];
        for chr in line.chars() {
            row.push(chr.to_string().parse::<usize>().unwrap() + 1)
        }
        grid.push(row);
    }
    // assumed square
    grid
}
