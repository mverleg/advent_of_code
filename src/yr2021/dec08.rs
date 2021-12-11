use ::std::collections::HashMap;
use ::std::fmt;
use ::std::fs::read_to_string;

use ::itertools::Itertools;

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run2();
    println!("{}", res);
}

fn run1() -> u64 {
    parse_input().iter()
        .map(|(_, outp)| outp)
        .map(|outp| outp.iter()
            .map(|word| word.len())
            .filter(|len| len <= &4 || len == &7)
            .count())
        .sum::<usize>() as u64
}

fn run2() -> u64 {
    parse_input().iter()
        .map(|(ptrns, outp)| row(ptrns, outp))
        .take(1)  //TODO @mark: TEMPORARY! REMOVE THIS!
        .sum()
}

fn row(ptrns: &[Vec<usize>], outp: &[Vec<usize>]) -> u64 {
    let mapping = find_mapping(ptrns);
    apply_mapping(mapping, outp)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Incomplete,
    Solved,
    Conflict,
}

#[derive(Debug)]
struct Possible {
    // would be cooler with bitmap, but this is easier and fast enough
    grid: [[bool; 7]; 7],
}

impl Possible {
    fn new() -> Self {
        Possible {
            grid: [[true; 7]; 7],
        }
    }

    fn without_goods(bads: &[usize], goods: &[usize]) -> Self {
        eprintln!("  - {:?} x {:?}", &bads, &goods);  //TODO @mark: TEMPORARY! REMOVE THIS!
        let mut pos = Possible::new();
        for bad in bads {
            for good in goods {
                pos.disable(*bad, *good)
            }
        }
        pos
    }

    fn disable(&mut self, bad: usize, good: usize) {
        self.grid[bad][good] = false;
        self.grid[good][bad] = false;
    }

    fn and(&mut self, other: Self) -> Self {
        self.combine(other, |a, b| a && b)
    }

    fn or(&mut self, other: Self) -> Self {
        self.combine(other, |a, b| a || b)
    }

    fn combine(&mut self, other: Self, op: fn(bool, bool) -> bool) -> Self {
        let mut pos = Possible::new();
        for bad in 0 .. 7 {
            for good in 0 .. 7 {
                pos.grid[bad][good] = op(self.grid[bad][good], other.grid[bad][good])
            }
        }
        pos
    }

    fn state(&self) -> State {
        for bad in 0 .. 7 {
            let mut sum = 0;
            for good in 0 .. 7 {
                if self.grid[bad][good] {
                    sum += 1
                }
            }
            if sum == 0 {
                return State::Conflict
            }
            if sum > 1 {
                return State::Incomplete
            }
        }
        State::Solved
    }
}

impl fmt::Display for Possible {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  + ");
        for good in 0 .. 7 {
            write!(f, "{} ", good);
        }
        writeln!(f, "");
        for bad in 0 .. 7 {
            write!(f, "{} | ", bad);
            for good in 0 .. 7 {
                if self.grid[bad][good] {
                    write!(f, "X ")?
                } else {
                    write!(f, "  ")?
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

fn find_mapping(ptrns: &[Vec<usize>]) -> Possible {
    let mut current = Possible::new();
    for ptrn in ptrns {
        eprintln!(">> {:?}", &ptrn);  //TODO @mark: TEMPORARY! REMOVE THIS!
        current = current.and(match ptrn.len() {
            2 => Possible::without_goods(ptrn, &[0, 1, 3, 4, 6,]),  // 1
            3 => Possible::without_goods(ptrn, &[1, 3, 4, 6,]),  // 7
            4 => Possible::without_goods(ptrn, &[0, 4, 6,]),  //4
            5 => Possible::without_goods(ptrn, &[1, 5,]).or(  // 2
                Possible::without_goods(ptrn, &[1, 4,])).or(  // 3
                Possible::without_goods(ptrn, &[2, 4,])),  // 5
            6 => Possible::without_goods(ptrn, &[3,]).or(  // 0
                Possible::without_goods(ptrn, &[2,])).or(  // 6
                Possible::without_goods(ptrn, &[4,])),  // 9
            7 => continue,  // 8
            _ => unreachable!(),
        });
        eprintln!("{}", &current);  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
    eprintln!("{}", &current);
    assert!(current.state() == State::Solved);
    current
}

fn apply_mapping(mapping: Possible, outp: &[Vec<usize>]) -> u64 {
    unimplemented!()
}

fn char2pos(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

fn parse_input() -> Vec<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
    let inps = get_lines("data/2021/dec08.txt").iter()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(ptrns, outp)| (space_sep_chars(ptrns, true), space_sep_chars(outp, false)))
        .collect::<Vec<_>>();
    inps
}

fn space_sep_chars(txt: &str, sort: bool) -> Vec<Vec<usize>> {
    let mut words: Vec<Vec<usize>> = txt.split(" ")
        .map(|word| word.chars()
            .map(char2pos)
            .collect::<Vec<_>>())
        .collect();
    if sort {
        words.sort_by_key(|word| match word.len() {
            7 => 1,
            x => x,
        })
    }
    words
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
