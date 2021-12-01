use ::lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub use grammar::*;

#[test]
fn test_real_exprs() {
    use crate::parse::grammar::RealExprsParser;
    let reals = RealExprsParser::new()
        .parse("1.2 3 4.5 6 7")
        .unwrap();
}
