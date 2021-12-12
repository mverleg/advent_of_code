use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn part_a() {
    let res = run();
    println!("{}", res);
}

pub fn part_b() {
    let res = run();
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Res {
    id: u32,
    price: u32,
}

fn run() -> u64 {
    let mut grid = get_grid("data/2021/dec11.txt");
    let mut flash_cnt = 0;

    for t in 0 .. 100 {
        // step 1 (increase) and 2a (flash)
        for x in 0 .. grid.len() {
            for y in 0 .. grid[0].len() {
                grid[x][y] += 1;
                flash(&mut grid, x, y)
            }
        }

        // step 2b (count flashes) and 3 (reset)
        for x in 0 .. grid.len() {
            for y in 0 .. grid[0].len() {
                if grid[x][y] > 9 {
                    flash_cnt += 1;
                    grid[x][y] = 0
                }
            }
        }
    }

    flash_cnt
}

fn flash(grid: &mut [Vec<u8>], x: usize, y: usize) {
    unimplemented!()
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
