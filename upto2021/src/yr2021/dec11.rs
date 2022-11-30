use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

pub fn part_a() {
    let res = run1(100);
    println!("{}", res);
}

pub fn part_b() {
    let res = run2();
    println!("{}", res);
}

fn run1(max_iter: i32) -> u64 {
    let mut grid = get_grid("data/2021/dec11.txt");
    let mut flash_cnt = 0;

    for t in 0 .. max_iter {
        // step 1 (increase) and 2a (flash)
        for_grid(&mut grid, |g, x, y| {
            flash(g, x, y)
        });

        // step 2b (count flashes) and 3 (reset)
        for_grid_val(&mut grid, |val| {
            if *val > 9 {
                flash_cnt += 1;
                *val = 0
            }
        });
    }

    flash_cnt
}

fn run2() -> u64 {
    let mut grid = get_grid("data/2021/dec11.txt");
    let grid_size = grid.len() * grid[0].len();
    let mut iter_nr = 0;
    loop {
        iter_nr += 1;
        let mut iter_flash_count = 0;

        // step 1 (increase) and 2a (flash)
        for_grid(&mut grid, |g, x, y| {
            flash(g, x, y)
        });

        // step 2b (count flashes) and 3 (reset)
        for_grid_val(&mut grid, |val| {
            if *val > 9 {
                iter_flash_count += 1;
                *val = 0
            }
        });

        if iter_flash_count == grid_size {
            return iter_nr
        }
    }
}

fn for_grid(grid: &mut [Vec<u8>], mut op: impl FnMut(&mut [Vec<u8>], usize, usize)) {
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            op(grid, x, y)
        }
    }
}

fn for_grid_val(grid: &mut [Vec<u8>], mut op: impl FnMut(&mut u8)) {
    for_grid(grid, |g, x, y| op(&mut g[x][y]))
}

fn print_grid(grid: &mut Vec<Vec<u8>>) {
    // print
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            print!("{} ", grid[x][y]);
        }
        println!("")
    }
    println!("")
}

fn flash(grid: &mut [Vec<u8>], x: usize, y: usize) {
    let value = &mut grid[x][y];
    // eprintln!("visit {}, {} at {}", x, y, value);
    if *value > 9 {
        return
    }
    *value += 1;
    // eprintln!("  +1");
    if *value <= 9 {
        return
    }
    // eprintln!("  flash");
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

#[test]
fn simple_noflash() {
    let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
    for_grid(&mut grid, flash);
    assert_eq!(grid, vec![vec![2, 3, 4], vec![5, 6, 7], vec![8, 9, 1]]);
}

#[test]
fn simple_flash() {
    let mut grid = vec![vec![0, 0, 0], vec![0, 9, 0], vec![0, 0, 0]];
    for_grid(&mut grid, flash);
    assert_eq!(grid, vec![vec![2, 2, 2], vec![2, 10, 2], vec![2, 2, 2]]);
}

#[test]
fn surrounded_flash() {
    let mut grid = vec![vec![9, 9, 9], vec![9, 1, 9], vec![9, 9, 9]];
    for_grid(&mut grid, flash);
    assert_eq!(grid, vec![vec![10, 10, 10], vec![10, 10, 10], vec![10, 10, 10]]);
}

#[test]
fn linear_flashes() {
    let mut grid = vec![vec![8, 8, 8, 8, 8, 9]];
    for_grid(&mut grid, flash);
    assert_eq!(grid, vec![vec![10, 10, 10, 10, 10, 10]]);
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
