#![allow(dead_code)]
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate pest_derive;

extern crate regex;

mod util;
mod problems;

fn main() {
    problems::day19::stage_2::run();
}
