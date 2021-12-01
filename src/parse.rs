use ::std::cmp::max;
use ::std::fmt::Write;
use ::std::process::exit;

use ::lalrpop_util::lalrpop_mod;
use ::lalrpop_util::ParseError;
use ::lalrpop_util::lexer::Token;

pub use grammar::*;
use crate::ast::Expr;

lalrpop_mod!(pub grammar);

/// Parsing with lalrpop grammar. Example:
/// ```
//  let reals = parse("test", "1.2 3 4.5 6 7", |c| RealExprsParser::new().parse(c));
/// ```
pub fn parse<T>(identifier: &str, code: &str, parse: fn(&str) -> Result<T, ParseError<usize, Token<'_>, &'static str>>) -> T {
    match try_parse(identifier, code, parse) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            panic!("parsing {} failed", identifier);
        }
    }
}

#[rustfmt::skip]
fn try_parse<T>(identifier: &str, code: &str, parse: fn(&str) -> Result<T, ParseError<usize, Token<'_>, &'static str>>) -> Result<T, String> {
    match parse(code) {
        Ok(ast) => Ok(ast),
        Err(err) => Err(match err {
            ParseError::InvalidToken { location } => {
                let (line, col) = source_line_col(code, location);
                format!("Invalid code in {}:{}:{}\n{}", identifier,
                        line + 1, col + 1, source_loc_repr(code, line, col, 1))
            }
            ParseError::UnrecognizedEOF { location, expected } => {
                let (line, col) = source_line_col(code, location);
                format!("Unexpected end in {}:{}:{}\n{}{}", identifier, line + 1, col + 1,
                        source_loc_repr(code, line, col, 1), fmt_expected_tokens(&expected)
                )
            }
            ParseError::UnrecognizedToken {
                token: (start, _, end),
                expected,
            } => {
                let (line, col) = source_line_col(code, start);
                format!("Unexpected code in {}:{}:{}\n{}{}", identifier, line + 1, col + 1,
                        source_loc_repr(code, line, col, max(1, end - start)), fmt_expected_tokens(&expected)
                )
            }
            ParseError::ExtraToken {
                token: (start, _, end),
            } => {
                let (line, col) = source_line_col(code, start);
                format!("Invalid token in {}:{}:{}\n{}", identifier, line + 1, col + 1,
                        source_loc_repr(code, line, col, max(1, end - start))
                )
            }
            ParseError::User { error } => {
                format!("Error in {}: {}", identifier, error)
            }
        }),
    }
}

fn fmt_expected_tokens(tokens: &[String]) -> String {
    if tokens.is_empty() {
        "Do not know what to expect at this position".to_owned()
    } else if tokens.len() == 1 {
        format!("Expected: {}", tokens[0])
    } else {
        format!("Expected one of: {}", tokens.join(", "))
    }
}

fn source_line_col(code: &str, start: usize) -> (usize, usize) {
    let mut err_line_nr = 0;
    let mut err_char_in_line = 0;
    let mut char_nr = 0;
    for line in code.lines() {
        if char_nr + line.len() >= start {
            err_char_in_line = start - char_nr;
            break;
        }
        char_nr += line.len() + 1;
        err_line_nr += 1;
    }
    (err_line_nr, err_char_in_line)
}

#[rustfmt::skip]
fn source_loc_repr(code: &str, err_line: usize, err_col: usize, len: usize) -> String {
    assert!(len >= 1);
    let mut locator = String::with_capacity(160);
    for (line_nr, line) in code.lines().enumerate() {
        if line_nr + 2 > err_line {
            writeln!(locator, "{:3} | {}", line_nr + 1, line);
        }
        if line_nr == err_line {
            let end_loc = if len > 1 { format!("-{}", err_col + len) } else { "".to_owned() };
            writeln!(locator, "      {}{} {}{}", " ".repeat(err_col), "^".repeat(len), err_col + 1, end_loc);
        }
        if line_nr > err_line + 2 {
            break;
        }
    }
    locator
}

#[test]
fn test_real_exprs() {
    use crate::parse::grammar::RealExprsParser;
    let code = "1.2, 3, -4.5, -6, 7";
    let reals = parse("test", code, |c| RealExprsParser::new().parse(c));
    let expected = vec![1.2, 3., -4.5, -6., 7.].into_iter()
        .map(|r| Box::new(Expr::Val(r)))
        .collect::<Vec<Box<Expr<f64>>>>();
    assert_eq!(reals, expected);
}
