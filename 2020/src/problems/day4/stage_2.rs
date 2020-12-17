use crate::util::read_lines;
use super::passport::{Passport};

pub fn run() {
    let mut current = "".to_owned();
    let mut passports = Vec::new();
    let lines = read_lines("./src/problems/day4/input.txt").unwrap();
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            passports.push(Passport::from(&current));
            current = "".to_owned();
        } else {
            current.push_str(" ");
            current.push_str(&line);
        }
    }
    passports.push(Passport::from(&current));

    println!("Found {} valid passports.", passports.iter().filter(|p| p.is_valid_stage2()).count())
}