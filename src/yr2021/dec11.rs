use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

const T: usize = 3;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run1();
    println!("{}", res);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Res {
    id: u32,
    price: u32,
}

fn run1() -> u64 {
    let mut grid = get_grid("data/2021/dec11.txt");
    let mut flash_cnt = 0;

    for t in 0 .. T {
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

        // print
        for x in 0 .. grid.len() {
            for y in 0..grid[0].len() {
                print!("{} ", grid[x][y]);
            }
            println!("")
        }
        dbg!(flash_cnt);
    }

    flash_cnt
}

fn flash(grid: &mut [Vec<u8>], x: usize, y: usize) {
    if grid[x][y] > 10 {
        return
    }
    grid[x][y] += 1;
    if grid[x][y] <= 9 {
        return
    }
    if x > 0 {
        flash(grid, x - 1, y);
        if y > 0 {
            flash(grid, x - 1, y - 1)
        }
        if y < grid[0].len() - 1 {
            flash(grid, x - 1, y + 1);
        }
    }
    if x < grid.len() - 1 {
        flash(grid, x + 1, y);
        if y > 0 {
            flash(grid, x + 1, y - 1)
        }
        if y < grid[0].len() - 1 {
            flash(grid, x + 1, y + 1);
        }
    }
    if y > 0 {
        flash(grid, x, y - 1);
    }
    if y < grid[0].len() - 1 {
        flash(grid, x, y + 1);
    }
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
