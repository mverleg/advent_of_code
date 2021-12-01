
use ::std::fs::read_to_string;

use crate::parse::parse;

pub fn dec06a() {
    let res = run();
    println!("{}", res);
}

pub fn dec06b() {
    let res = run();
    println!("{}", res);
}

fn run() -> u64 {
    let inp = read_to_string("data/2020/dec06ex.txt").unwrap();
    use crate::parse::grammar::Yr2020Dec06Parser;
    let lines = parse("Yr2020Dec06", &inp, |c| Yr2020Dec06Parser::new().parse(c));

    unimplemented!()
}
