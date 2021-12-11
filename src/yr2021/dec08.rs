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
        .map(|(ptrns, outp)| (find_mapping(ptrns), outp))
        .map(|(mapping, outps)| outps.iter()
            .map(|outp| interpret(outp, mapping))
            .sum::<usize>())
        .sum::<usize>() as u64
}

fn find_mapping(ptrns: &Vec<Vec<char>>) -> [char; 9] {
    ptrns.iter()
        .map(|ptrn| possibilities(ptrn))
        .inspect(|possib| eprintln!("{:?}", possib))
        .reduce(|a, b| unimplemented!());

    unimplemented!()
}

fn possibilities(word: &Vec<char>) -> Vec<(i32, [Option<char>; 9])> {
    match word.len() {
        2 => vec![
            (1, [None, None, Some(word[0]), None, None, None, None, Some(word[1]), None,]),
            (1, [None, None, Some(word[1]), None, None, None, None, Some(word[0]), None,]),
        ],
        3 => unimplemented!(),
        4 => unimplemented!(),
        5 => unimplemented!(),
        6 => unimplemented!(),
        7 => vec![
            (7, [Some(word[0]), None, Some(word[1]), None, None, None, None, Some(word[2]), None,]),
            (7, [Some(word[0]), None, Some(word[2]), None, None, None, None, Some(word[1]), None,]),
            //TODO @mark: more
        ],
        _ => unreachable!(),
    }
}

fn char2pos(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

fn interpret(outp: &Vec<char>, mapping: [char; 9]) -> usize {
    unimplemented!()
}

fn parse_input() -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    let inps = get_lines("data/2021/dec08.txt").iter()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(ptrns, outp)| (space_sep_ints(ptrns), space_sep_ints(outp)))
        .collect::<Vec<_>>();
    inps
}

fn space_sep_ints(txt: &str) -> Vec<Vec<char>> {
    txt.split(" ")
        .map(|word| word.chars().collect::<Vec<_>>())
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
