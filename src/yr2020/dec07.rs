use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::fs::read_to_string;
use std::assert_matches::assert_matches;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::regex::Regex;
use nom::bytes::complete::tag;
use nom::character::streaming::alphanumeric1;
use nom::error::{context, convert_error, VerboseError};
use nom::{Err, IResult};
use nom::branch::alt;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res};
use nom::Err::Incomplete;
use nom::Needed::Size;
use nom::sequence::{pair, separated_pair, tuple};

pub fn part_a() {
    let mut map = HashMap::<String, HashSet<String>>::new();
    for line in get_lines("data/2020/dec07.txt") {
        let (outer, inner) = line.split_once(" bags contain ").unwrap();
        if inner == "no other bags." {
            continue;
        }
        let inners = inner.split(", ")
            .map(|bag| {
                let parts = bag.split(" ").collect::<Vec<_>>();
                format!("{} {}", parts[1], parts[2])
            })
            .collect::<Vec<_>>();
        for inner in inners {
            if !map.contains_key(&inner) {
                map.insert(inner.clone(), HashSet::new());
            }
            map.get_mut(&inner).unwrap().insert(outer.to_owned());
        }
    }
    let res = find_outer(&map, "shiny gold").len() as u64;
    println!("{}", res);
}

pub fn part_b() {
    let content = read_to_string("data/2020/dec07.txt").unwrap();
    bag_color(&content);


    let res = 0;
    println!("{}", res);
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Bag<'a> {
    adj: &'a str,
    color: &'a str,
}

impl<'a> Bag<'a> {
    fn new(adj: &'a str, color: &'a str) -> Self {
        Bag { adj, color }
    }
}

fn bag_color(input: &str) -> Res<&str, Bag> {
    context("bag",
            map(
                tuple((
                    alphanumeric1,
                    space1,
                    alphanumeric1,
                    alt((tag(" bags"), tag(" bag"))),
                )),
                |(adj, _, color, _)| Bag::new(adj, color),
            ),
    )(input)
}

fn bag_count(input: &str) -> Res<&str, (u32, Bag)> {
    context("count",
            map(
                tuple((
                    parse_u32,
                    space1,
                    bag_color,
                )),
                |(cnt, _, bag)| (cnt, bag),
            ),
    )(input)
}

fn parse_u32(input: &str) -> Res<&str, u32> {
    context("positive number", map(digit1, |cnt: &str| cnt.parse::<u32>().unwrap()))(input)
}

#[test]
fn bag_color_test() {
    assert_eq!(bag_color("light blue bags!"), Ok(("!", Bag { adj: "light", color: "blue" })));
    assert_matches!(bag_color("orange"), Err(Incomplete(_)));
}

#[test]
fn bag_count_test() {
    assert_eq!(bag_count("1 light blue bag."), Ok((".", (1, Bag { adj: "light", color: "blue" }))));
    assert_eq!(bag_count("3 light blue bags."), Ok((".", (3, Bag { adj: "light", color: "blue" }))));
    let inp = "1 light blue sky.";
    match bag_count(inp).unwrap_err() {
        Err::Error(err) =>
            assert!(convert_error(inp, err).contains("at line 1, in Tag:")),
        _ => panic!(),
    }
}

fn find_outer(map: &HashMap<String, HashSet<String>>, color: &str) -> HashSet<String> {
    let mut set = map.get(color)
        .map(|s| s.clone())
        .unwrap_or_else(|| HashSet::new());
    for outer_col in set.clone() {
        // if set.contains(&outer_col) {
        //     continue
        // }
        set.extend(find_outer(map, &outer_col))
    }
    set
}

fn get_lines(pth: &str) -> Vec<String> {
    let content = read_to_string(pth).unwrap();
    content
        .lines()
        .filter(|ln| !ln.trim().is_empty())
        .map(|ln| ln.to_owned())
        .collect::<Vec<_>>()
}
