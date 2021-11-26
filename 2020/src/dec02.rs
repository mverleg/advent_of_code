use ::std::fs::read_to_string;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s+(\w):\s+(\w+)").unwrap();
}

pub fn dec02a() {
    let valid_cnt = read_to_string("dec02.txt")
        .unwrap()
        .lines()
        .filter(|ln| !ln.is_empty())
        .map(|ln| parse(ln))
        .filter(|pwd| validate(pwd))
        .count();
    println!("{}", valid_cnt);
}

#[derive(Debug)]
struct Pwd {
    min: u32,
    max: u32,
    letter: char,
    pass: String,
}

fn parse(line: &str) -> Pwd {
    let groups = RE.captures(line).unwrap();
    Pwd {
        min: groups[1].parse().unwrap(),
        max: groups[2].parse().unwrap(),
        letter: groups[3].chars().next().unwrap(),
        pass: groups[4].to_owned(),
    }
}

fn validate(pwd: &Pwd) -> bool {
    let cnt = pwd.pass.chars()
        .filter(|c| c == &pwd.letter)
        .count() as u32;
    return cnt >= pwd.min && cnt <= pwd.max;
}
