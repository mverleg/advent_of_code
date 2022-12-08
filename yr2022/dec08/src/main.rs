use ::std::fs::read_to_string;

// didn't measure time, many interruptions

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
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    let mut vis_cnt = 0;
    for (i, row) in grid.iter().enumerate() {
        let mut highest = 0;
        for (j, val) in row.iter().enumerate() {
            if *val > highest {
                highest = *val;
                if !seen[i][j] {
                    vis_cnt += 1;
                    seen[i][j] = true;
                }
            }
        }
        let mut highest = 0;
        for (j, val) in row.iter().enumerate().rev() {
            if *val > highest {
                highest = *val;
                if !seen[i][j] {
                    vis_cnt += 1;
                    seen[i][j] = true;
                }
            }
        }
    }
    for j in 0 .. grid[0].len() {
        let mut highest = 0;
        for i in 0..grid.len() {
            let val = &mut grid[i][j];
            if *val > highest {
                highest = *val;
                if !seen[i][j] {
                    vis_cnt += 1;
                    seen[i][j] = true;
                }
            }
        }
        let mut highest = 0;
        for i in (0..grid.len()).rev() {
            let val = &mut grid[i][j];
            if *val > highest {
                highest = *val;
                if !seen[i][j] {
                    vis_cnt += 1;
                    seen[i][j] = true;
                }
            }
        }
    }
    for row in seen.iter() {
        for cell in row.iter() {
            if *cell {
                eprint!(".");
            } else {
                eprint!("X");
            }
        }
        eprintln!("");
    }
    vis_cnt
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
