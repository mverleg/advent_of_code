use ::std::fs::read_to_string;

// didn't measure time, many interruptions

fn main() {
    let data = read_to_string("data.txt").unwrap();
    //println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}

fn part_a(data: &str) -> usize {
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
    for j in 0..grid[0].len() {
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

fn part_b(data: &str) -> usize {
    let mut grid = parse_grid(data);
    let mut highest_scenic = (0, 0, 0);
    for a in 1..grid.len() - 1 {
        for b in 1..grid[0].len() - 1 {
            let mut scenic: usize = 1;
            let start = grid[a][b];

            let mut vis_cnt = 0;
            for i in 0..a-1 {
                let val = &mut grid[i][b];
                if *val >= start {
                    break
                }
                vis_cnt += 1;
            }
            scenic *= vis_cnt;

            let mut vis_cnt = 0;
            for i in a+1..grid.len() {
                let val = &mut grid[i][b];
                if *val >= start {
                    break
                }
                vis_cnt += 1;
            }
            scenic *= vis_cnt;

            let mut vis_cnt = 0;
            for j in 0..b-1 {
                let val = &mut grid[a][j];
                if *val >= start {
                    break
                }
                vis_cnt += 1;
            }
            scenic *= vis_cnt;

            let mut vis_cnt = 0;
            for j in b+1..grid[0].len() {
                let val = &mut grid[a][j];
                if *val >= start {
                    break
                }
                vis_cnt += 1;
            }
            scenic *= vis_cnt;

            if scenic > highest_scenic.0 {
                eprintln!("update {} -> {scenic}", highest_scenic.0);  //TODO @mark: TEMPORARY! REMOVE THIS!
                highest_scenic = (scenic, a, b);
            }
        }
    }
    assert!(highest_scenic.0 >= 0);
    eprintln!("{} for {},{}", highest_scenic.0, highest_scenic.1, highest_scenic.2);
    highest_scenic.0 as usize
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
