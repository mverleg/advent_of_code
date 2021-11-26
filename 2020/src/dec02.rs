use ::std::fs::read_to_string;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+-\d+\s+\w:\s+\w+").unwrap();
}

#[test]
fn dec02a() {
    let valid_cnt = read_to_string("../dec02.txt")
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
    unimplemented!()
}

fn validate(pwd: &Pwd) -> bool {
    unimplemented!()
}
