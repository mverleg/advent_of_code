#![allow(unused)]

mod template;
mod ast;
mod parse;
mod yr2020;
mod yr2021;

use yr2021::*;
use crate::template::dec00a;

fn main() {
    dec01::dec01a();
    dec01::dec01b();
    dec00a()
}
