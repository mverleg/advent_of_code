use ::std::collections::HashMap;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();
}

pub fn part_a() {
    let res = run1();
    println!("{}", res);
}

pub fn part_b() {
    let res = run2();
    println!("{}", res);
}

const SZ: usize = 5;
const DONE: u8 = u8::MAX;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Board {
    nrs: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(nrs: Vec<Vec<u8>>) -> Self {
        Board { nrs }
    }

    pub fn mark(&mut self, pick: u8) {
        assert!(pick != DONE);
        for i in 0..SZ {
            for j in 0..SZ {
                if self.nrs[i][j] == pick {
                    assert!(self.nrs[i][j] != DONE);
                    self.nrs[i][j] = DONE;
                }
            }
        }
    }

    pub fn is_win(&self) -> bool {
        for i in 0..SZ {
            let mut all = true;
            for j in 0..SZ {
                if self.nrs[i][j] != DONE {
                    all = false;
                    break;
                }
            }
            if all {
                return true;
            }
        }

        for j in 0..SZ {
            let mut all = true;
            for i in 0..SZ {
                if self.nrs[i][j] != DONE {
                    all = false;
                    break;
                }
            }
            if all {
                return true;
            }
        }

        false
    }

    pub fn remaining_score(&self) -> u32 {
        let mut score: u32 = 0;
        for i in 0..SZ {
            for j in 0..SZ {
                if self.nrs[i][j] != DONE {
                    score += self.nrs[i][j] as u32;
                }
            }
        }
        score
    }
}

fn run1() -> u32 {
    let (picks, mut boards) = load_boards();

    for pick in picks {
        for board in &mut boards {
            board.mark(pick);
            if board.is_win() {
                return board.remaining_score() * (pick as u32)
            }
        }
    }

    unreachable!()
}

fn run2() -> u32 {
    let (picks, mut boards) = load_boards();

    for pick in picks {
        let mut not_win_boards = vec![];
        let is_last = boards.len() == 1;
        for mut board in &mut boards {
            board.mark(pick);
            if board.is_win() {
                // assumes there is only one last board, i.e. not two winning in the same round
                if is_last {
                    return board.remaining_score() * (pick as u32)
                }
                continue;
            }
            not_win_boards.push(board.clone());
        }
        boards = not_win_boards;
    }

    unreachable!()
}

fn load_boards() -> (Vec<u8>, Vec<Board>) {
    let lines = get_lines("data/2021/dec04.txt");
    let picks = lines[0].split(",")
        .map(|nr| nr.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut current = vec![];
    let mut boards = vec![];
    for line in lines.iter().skip(2) {
        if line.is_empty() {
            assert!(!current.is_empty());
            boards.push(Board::new(current.clone()));
            current.clear();
            continue
        }
        let line_nrs = line.split_ascii_whitespace()
            .map(|cell| cell.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        current.push(line_nrs)
    }
    if !current.is_empty() {
        boards.push(Board::new(current));
    }
    (picks, boards)
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
