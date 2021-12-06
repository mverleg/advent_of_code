use ::std::collections::HashMap;
use ::std::fs::read_to_string;
use std::cmp::{max, min};

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::error::VerboseError;
use nom::IResult;
use nom::sequence::tuple;

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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn run() -> u64 {
    let lines = get_lines("data/2021/dec05.txt").into_iter()
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    let grid = build_grid(lines);

    let mut count: u64 = 0;
    for row in grid {
        for cell in row {
            if cell > 1 {
                count += 1
            }
        }
    }

    count
}

fn build_grid(lines: Vec<Res>) -> Vec<Vec<usize>> {
    let x_max = lines.iter().map(|res| max(res.x1, res.x2)).max().unwrap() + 1;
    let y_max = lines.iter().map(|res| max(res.y1, res.y2)).max().unwrap() + 1;
    let mut grid = vec![vec![0usize; y_max]; x_max];
    for line in lines {
        if line.x1 == line.x2 && line.y1 == line.y2 {
            unimplemented!()
        }
        if line.x1 == line.x2 {
            for y in min(line.y1, line.y2) ..= max(line.y1, line.y2) {
                grid[line.x1][y] += 1
            }
        } else if line.y1 == line.y2 {
            for x in min(line.x1, line.x2) ..= max(line.x1, line.x2) {
                grid[x][line.y1] += 1
            }
        } else {
            // Skip for part a
            continue
        }
    }

    print_grid(&grid);

    grid
}

fn print_grid(grid: &[Vec<usize>]) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[x][y] != 0 {
                print!("{}", grid[x][y]);
            } else {
                print!(".");
            }
        }
        println!(";")
    }
}

fn parse_line(line: &str) -> Res {
    let mut parser = map(tuple((digit1, tag(","), digit1, tag(" -> "), digit1, tag(","), digit1)),
        |(x1, _, y1, _, x2, _, y2): (&str, &str, &str, &str, &str, &str, &str)| Res {
            x1: x1.parse::<usize>().unwrap(),
            y1: y1.parse::<usize>().unwrap(),
            x2: x2.parse::<usize>().unwrap(),
            y2: y2.parse::<usize>().unwrap()
        });
    let res: IResult<&str, Res, VerboseError<&str>> = parser(line);
    res.unwrap().1
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
