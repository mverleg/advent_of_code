use ::std::collections::HashMap;
use ::std::fmt;
use ::std::fs::read_to_string;

use ::bitmaps::Bitmap;
use ::itertools::Itertools;

use crate::yr2021::dec08::State::Solved;

const DIGIT_SEGS: [&[usize]; 10] = [
    &[0, 1, 2, 4, 5, 6,],  // 0
    &[2, 5,],  // 1
    &[0, 2, 3, 4, 6,],  // 2
    &[0, 2, 3, 5, 6,],  // 3
    &[1, 2, 3, 5,],  //4
    &[0, 1, 3, 5, 6,],  // 5
    &[0, 1, 3, 4, 5, 6,],  // 6
    &[0, 2, 5,],  // 7
    &[0, 1, 2, 3, 4, 5, 6,],  // 8
    &[0, 1, 2, 3, 5, 6,],  // 9
];

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
        .sum()
}

fn row(ptrns: &[Vec<usize>], outp: &[Vec<usize>]) -> u64 {
    let possible = Possible::new();
    let mapping = find_mapping(ptrns, possible)
        .expect("top level `find_mapping` did not find a solution");
    apply_mapping(mapping, outp)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Incomplete,
    Solved,
    Conflict,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            State::Incomplete => "incomplete",
            State::Solved => "SOLVED",
            State::Conflict => "conflict",
        })
    }
}

#[derive(Debug)]
struct Possible {
    // would be cooler with bitmap, but this is easier and fast enough
    bits: Bitmap<49>,
}

impl Possible {
    fn new() -> Self {
        Possible {
            bits: Bitmap::mask(7 * 7),
        }
    }

    fn keep(bads: &[usize], goods: &[usize]) -> Self {
        assert!(bads.len() == goods.len());
        //eprintln!("  - {:?} x {:?}", &bads, &goods);  //TODO @mark: TEMPORARY! REMOVE THIS!
        let mut pos = Possible::new();
        for bad in bads {
            for good in 0 .. 7 {
                pos.set(*bad, good, false);
            }
        }
        for good in goods {
            for bad in 0 .. 7 {
                pos.set(bad, *good, false);
            }
        }
        for bad in bads {
            for good in goods {
                pos.set(*bad, *good, true);
            }
        }
        pos
    }

    fn calc_index(&self, bad: usize, good: usize) -> usize {
        debug_assert!(bad < 7);
        debug_assert!(good < 7);
        bad * 7 + good
    }

    fn get(&self, bad: usize, good: usize) -> bool {
        self.bits.get(self.calc_index(bad, good))
    }

    fn set(&mut self, bad: usize, good: usize, value: bool) {
        self.bits.set(self.calc_index(bad, good), value);
    }

    fn and(&self, other: Self) -> Self {
        Possible { bits: self.bits & other.bits }
    }

    fn or(&self, other: Self) -> Self {
        Possible { bits: self.bits | other.bits }
    }

    /// Mapping from each bad segment to the correct one.
    fn as_solved_vec(&self) -> Vec<usize> {
        debug_assert!(self.state() == State::Solved);
        (0 .. 7)
            .map(|bad| (0 .. 7)
                .map(|good| (good, self.get(bad, good)))
                .filter(|(_, is_true)| *is_true)
                .next().expect("no mapping, impossible for solved").0
            ).collect::<Vec<_>>()
    }

    fn state(&self) -> State {
        let mut is_solved = true;
        for bad in 0 .. 7 {
            let mut sum = 0;
            for good in 0 .. 7 {
                if self.get(bad, good) {
                    sum += 1
                }
            }
            if sum == 0 {
                return State::Conflict
            }
            if sum > 1 {
                is_solved = false;
            }
        }
        if is_solved {
            State::Solved
        } else {
            State::Incomplete
        }
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
                if self.get(bad, good) {
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

/// Return solved Possible when this branch has a solution, or () if it is a dead end.
fn find_mapping(ptrns: &[Vec<usize>], possible: Possible) -> Result<Vec<usize>, ()> {
    let state = possible.state();
    //eprintln!("{}state: {}", &possible, state);  //TODO @mark: TEMPORARY! REMOVE THIS!
    match state {
        State::Incomplete => {}
        State::Solved => {
            return Ok(possible.as_solved_vec())
        },
        State::Conflict => return Err(()),
    }
    if let [ptrn, rest @ ..] = ptrns {
        //eprintln!(">> {:?} (of {})\n", &ptrn, ptrns.len());  //TODO @mark: TEMPORARY! REMOVE THIS!
        handle_head(ptrn, rest, possible)
    } else {
        match state {
            State::Incomplete => Err(()),
            State::Solved => Ok(possible.as_solved_vec()),
            State::Conflict => Err(()),
        }
    }
}

fn handle_head(ptrn: &Vec<usize>, rest: &[Vec<usize>], possible: Possible) -> Result<Vec<usize>, ()> {
    match ptrn.len() {
        2 => find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[1]))),
        3 => find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[7]))),
        4 => find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[4]))),
        5 => find_single_ok(
            find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[2]))),
            find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[3]))),
            find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[5]))),
        ),
        6 => find_single_ok(
            find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[0]))),
            find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[6]))),
            find_mapping(rest, possible.and(Possible::keep(ptrn, DIGIT_SEGS[9]))),
        ),
        7 => find_mapping(rest, possible),  // 8
        _ => panic!("impossible"),
    }
}

fn find_single_ok<T, E>(first: Result<T, E>, second: Result<T, E>, third: Result<T, E>) -> Result<T, E> {
    match (first, second, third) {
        (Err(first), Err(_), Err(_)) => Err(first),
        (Ok(val), Err(_), Err(_)) => Ok(val),
        (Err(_), Ok(val), Err(_)) => Ok(val),
        (Err(_), Err(_), Ok(val)) => Ok(val),
        _ => panic!("found multiple successful results"),
    }
}

fn apply_mapping(mapping: Vec<usize>, outps: &[Vec<usize>]) -> u64 {
    let mut total = 0;
    for outp in outps {
        let decoded = outp.iter()
            .map(|seg| mapping[*seg])
            .sorted()
            .collect::<Vec<usize>>();
        let digit = DIGIT_SEGS.iter()
            .enumerate()
            .filter(|(nr, segs)| **segs == decoded.as_slice())
            .next().expect("found an unrecognized output pattern").0;
        total *= 10;
        total += digit;
    }
    total as u64
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
