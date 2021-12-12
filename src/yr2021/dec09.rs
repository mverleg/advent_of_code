use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

pub fn part_a() {
    let res = run();
    println!("{}", res);
}

pub fn part_b() {
    let res = run();
    println!("{}", res);
}

fn run() -> u64 {
    let grid = get_grid("data/2021/dec09.txt");
    let mut minimum_scores = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            let is_hor_min = (y == 0 || val < &row[y - 1]) && (y == row.len() - 1 || val < &row[y + 1]);
            let is_ver_min = (x == 0 || val < &grid[x - 1][y]) && (x == grid.len() - 1 || val < &grid[x + 1][y]);
            if is_hor_min && is_ver_min {
                minimum_scores += 1 + (*val as u64)
            }
        }
    }
    minimum_scores
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
