use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run2();
    println!("{}", res);
}

fn run1() -> u64 {
    let grid = get_grid("data/2021/dec09.txt");
    let mut minimum_scores = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            let is_ver_min = (y == 0 || val < &row[y - 1]) && (y == row.len() - 1 || val < &row[y + 1]);
            let is_hor_min = (x == 0 || val < &grid[x - 1][y]) && (x == grid.len() - 1 || val < &grid[x + 1][y]);
            if is_hor_min && is_ver_min {
                minimum_scores += 1 + (*val as u64)
            }
        }
    }
    minimum_scores
}

fn run2() -> u64 {
    let grid = get_grid("data/2021/dec09.txt");
    let mut basin_ids = vec![vec![0; grid[0].len()]; grid.len()];
    let mut basin_sizes = vec![];
    for (x, row) in grid.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            if *val == 9 {
                continue
            }
            if basin_ids[x][y] != 0 {
                continue
            }
            let next_basin_id = basin_sizes.len() + 1;
            let size = fill_basin(&grid, &mut basin_ids, x, y, next_basin_id);
            basin_sizes.push(size);
        }
    }
    assert!(basin_sizes.len() >= 3);
    basin_sizes.iter()
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn fill_basin(grid: &[Vec<u8>], basin_ids: &mut [Vec<usize>], x: usize, y: usize, id: usize) -> u64 {
    eprintln!("{}, {} = {}", x, y, id);  //TODO @mark: TEMPORARY! REMOVE THIS!
    if basin_ids[x][y] != 0 {
        return 0;
    }
    if grid[x][y] == 9 {
        return 0;
    }
    basin_ids[x][y] = id;
    let mut size = 0;
    if x > 0 {
        size += fill_basin(grid, basin_ids, x - 1, y, id);
    }
    if x < grid.len() - 1 {
        size += fill_basin(grid, basin_ids, x + 1, y, id);
    }
    unimplemented!();
    size + 1
}

fn get_grid(pth: &str) -> Vec<Vec<u8>> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.chars()
            .map(|c| (c as u8) - ('0' as u8))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
