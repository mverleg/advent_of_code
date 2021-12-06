#![feature(assert_matches)]
#![allow(unused)]

mod template;
mod ast;
mod parse;
mod yr2020;
mod yr2021;

fn main() {
    yr2021::dec05::part_a();
    yr2021::dec05::part_b();
}
