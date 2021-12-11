use ::std::collections::HashMap;
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
        .sum()
}

fn row(ptrns: &[Vec<usize>], outp: &[Vec<usize>]) -> u64 {
    let mapping = find_mapping(ptrns);
    apply_mapping(mapping, outp)
}

#[derive(Debug)]
struct Possible {
    grid: [[bool; 7]; 7]
}

impl Possible {
    fn new() -> Self {
        Possible {
            grid: [[true; 7]; 7],
        }
    }

    fn disable(&mut self, from: usize, to: usize) -> Self {
        unimplemented!()
    }

    fn and(&mut self, other: Self) -> Self {
        unimplemented!()
    }

    fn or(&mut self, other: Self) -> Self {
        unimplemented!()
    }
}

fn find_mapping(ptrns: &[Vec<usize>]) -> Possible {
    let mut current = Possible::new();
    for ptrn in ptrns {
        current = current.and(match ptrn.len() {
            2 => {
                return Possible::new()
            },
            3 => unimplemented!(),
            4 => unimplemented!(),
            5 => unimplemented!(),
            6 => unimplemented!(),
            7 => unimplemented!(),
            _ => unreachable!(),
        })
    }
    current
}

fn apply_mapping(mapping: Possible, outp: &[Vec<usize>]) -> u64 {
    unimplemented!()
}

// fn find_mapping(ptrns: &Vec<Vec<usize>>) -> [usize; 9] {
//     ptrns.iter()
//         .map(|ptrn| possibilities(ptrn))
//         .inspect(|possib| eprintln!("{:?}", possib))
//         .reduce(|a, b| unimplemented!());
//
//     unimplemented!()
// }
//
// fn possibilities(word: &Vec<usize>) -> Vec<(i32, [Option<usize>; 9])> {
//     match word.len() {
//         2 => vec![
//             (1, [None, None, Some(word[0]), None, None, None, None, Some(word[1]), None,]),
//             (1, [None, None, Some(word[1]), None, None, None, None, Some(word[0]), None,]),
//         ],
//         3 => unimplemented!(),
//         4 => unimplemented!(),
//         5 => unimplemented!(),
//         6 => unimplemented!(),
//         7 => vec![
//             (7, [Some(word[0]), None, Some(word[1]), None, None, None, None, Some(word[2]), None,]),
//             (7, [Some(word[0]), None, Some(word[2]), None, None, None, None, Some(word[1]), None,]),
//             //TODO @mark: more
//         ],
//         _ => unreachable!(),
//     }
// }

fn char2pos(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

fn parse_input() -> Vec<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
    let inps = get_lines("data/2021/dec08.txt").iter()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(ptrns, outp)| (space_sep_chars(ptrns), space_sep_chars(outp)))
        .collect::<Vec<_>>();
    inps
}

fn space_sep_chars(txt: &str) -> Vec<Vec<usize>> {
    txt.split(" ")
        .map(|word| word.chars()
            .map(char2pos)
            .collect::<Vec<_>>())
        .collect()
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
