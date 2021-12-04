use ::std::assert_matches::assert_matches;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::fs::read_to_string;

use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::nom::{Err, IResult};
use ::nom::branch::alt;
use ::nom::bytes::complete::tag;
use ::nom::character::complete::{digit1, space1};
use ::nom::character::complete::newline;
use ::nom::character::streaming::alphanumeric1;
use ::nom::combinator::{map, map_res};
use ::nom::combinator::opt;
use ::nom::Err::Incomplete;
use ::nom::error::{context, convert_error, VerboseError};
use ::nom::multi::many1;
use ::nom::Needed::Size;
use ::nom::sequence::{pair, separated_pair, tuple};
use ::regex::Regex;
use nom::multi::many0;

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
    let bags = lines(&content).into_iter()
        //.inspect(|(outer, inner)| println!("{:?}", outer))
        .collect::<HashMap<Bag, Vec<(u32, Bag)>>>();
    dbg!(bags.len());
    count_inner(&bags, &Bag { adj: "shiny", color: "gold" });
    let res = 0;
    println!("{}", res);
}

fn count_inner(bags: &HashMap<Bag, Vec<(u32, Bag)>>, bag: &Bag) -> u32 {
    match bags.get(bag) {
        Some(sub_bags) => sub_bags.iter()
            .map(|(cnt, sub_bag)| cnt + count_inner(bags, sub_bag))
            .sum(),
        None => 0
    }
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Bag<'a> {
    adj: &'a str,
    color: &'a str,
}

type Inner<'a> = Vec<(u32, Bag<'a>)>;

impl<'a> Bag<'a> {
    fn new(adj: &'a str, color: &'a str) -> Self {
        Bag { adj, color }
    }
}

fn parse_u32(input: &str) -> Res<&str, u32> {
    context("positive number", map(digit1, |cnt: &str| cnt.parse::<u32>().unwrap()))(input)
}

fn bag_color(input: &str) -> Res<&str, Bag> {
    context("bag", map(
        tuple((
            alphanumeric1,
            space1,
            alphanumeric1,
            alt((tag(" bags"), tag(" bag"))),
        )),
        |(adj, _, color, _)| Bag::new(adj, color),
    ))(input)
}

fn bag_count(input: &str) -> Res<&str, (u32, Bag)> {
    context("count", map(
        tuple((
            parse_u32,
            space1,
            bag_color,
            alt((tag(", "), tag(".")))
        )),
        |(cnt, _, bag, _)| (cnt, bag),
    ))(input)
}

fn no_bags(input: &str) -> Res<&str, Inner> {
    map(tag("no other bags."), |_| vec![])(input)
}

fn line(input: &str) -> Res<&str, (Bag, Inner)> {
    context("line", map(
        tuple((
            bag_color,
            tag(" contain "),
            alt((
                no_bags,
                many1(bag_count),
            )),
            opt(newline),
        )),
        |(outer, _, inner, _)| (outer, inner),
    ))(input)
}

fn lines(input: &str) -> Vec<(Bag, Inner)> {
    let tmp = many1(line)(input);  //TODO @mark: TEMPORARY! REMOVE THIS!
    let (rem, ast) = map(pair(many1(line), many0(newline)),
        |(bags, _)| bags)(&input).unwrap();
    assert!(rem.is_empty());
    ast
}

#[test]
fn bag_color_test() {
    assert_eq!(bag_color("light blue bags!"), Ok(("!", Bag { adj: "light", color: "blue" })));
    assert_matches!(bag_color("orange"), Err(Incomplete(_)));
}

#[test]
fn bag_count_test() {
    assert_eq!(bag_count("1 light blue bag.!"), Ok(("!", (1, Bag { adj: "light", color: "blue" }))));
    assert_eq!(bag_count("3 light blue bags.!"), Ok(("!", (3, Bag { adj: "light", color: "blue" }))));
    let inp = "1 light blue sky.";
    match bag_count(inp).unwrap_err() {
        Err::Error(err) =>
            assert!(convert_error(inp, err).contains("at line 1, in Tag:")),
        _ => panic!(),
    }
}

#[test]
fn line_test() {
    assert_eq!(line("faded blue bags contain no other bags."),
               Ok(("", (Bag { adj: "faded", color: "blue" }, vec![]))));
    assert_eq!(line("bright white bags contain 1 shiny gold bag."),
               Ok(("", (Bag { adj: "bright", color: "white" }, vec![(1, Bag { adj: "shiny", color: "gold" })]))));
    assert_eq!(line("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
               Ok(("", (Bag { adj: "shiny", color: "gold" }, vec![(1, Bag { adj: "dark", color: "olive" }), (2, Bag { adj: "vibrant", color: "plum" })]))));
}

#[test]
fn multi_line() {
    assert_eq!(lines("faded blue bags contain no other bags.\nbright white bags contain 1 shiny gold bag.\n\n"),
               vec![(Bag { adj: "faded", color: "blue" }, vec![]),
                    (Bag { adj: "bright", color: "white" }, vec![(1, Bag { adj: "shiny", color: "gold" })])]);
}

fn find_outer(map: &HashMap<String, HashSet<String>>, color: &str) -> HashSet<String> {
    let mut set = map.get(color)
        .map(|s| s.clone())
        .unwrap_or_else(|| HashSet::new());
    for outer_col in set.clone() {
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
