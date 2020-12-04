use super::password::{self, PasswordLine};

use crate::util::read_lines;


pub fn run() {
    let lines = read_lines("./src/problems/day2/input.txt").unwrap();
    let entries: Vec<PasswordLine> = lines.into_iter()
        .map(|entry| password::parse_line(&entry.unwrap()))
        .filter(|password| is_valid(password))
        .collect();
    println!("Found {} valid passwords.", entries.len())
}

fn is_valid(line: &PasswordLine) -> bool {
    let count = line.password.chars().filter(|x| *x == line.policy.character).count();
    return line.policy.from <= count && count <= line.policy.to;
}
