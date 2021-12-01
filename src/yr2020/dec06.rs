use ::std::fs::read_to_string;

use crate::parse::parse_file;
use crate::parse::grammar::Yr2020Dec06Parser;

pub fn dec06a() {
    let res = run();
    println!("{}", res);
}

pub fn dec06b() {
    let res = run();
    println!("{}", res);
}

fn run() -> u64 {
    let lines = parse_file("data/2020/dec06ex.txt", |c| Yr2020Dec06Parser::new().parse(c));
    for line in lines {
        println!("{:?}", line);
    }
    unimplemented!()
}
