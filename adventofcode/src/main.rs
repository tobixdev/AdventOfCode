#![allow(dead_code)]
#[macro_use] extern crate lazy_static;

extern crate regex;

mod util;
mod problems;

fn main() {
    problems::day17::stage_2::run();
}
