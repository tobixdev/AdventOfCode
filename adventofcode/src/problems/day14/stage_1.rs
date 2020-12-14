use std::collections::HashMap;
use regex::Regex;
use crate::util::read_lines;

pub fn run() {
    let lines = read_lines("./src/problems/day14/input.txt")
        .unwrap()
        .map(|l| l.unwrap());
    let mut memory = HashMap::new();
    let assignment_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut one_mask: u64 = 0x0;
    let mut zero_mask: u64 = 0xFFFFFFFFF; // 32-bit 1
    for line in lines {
        if line.starts_with("mask = ") {
            let (new_one_mask, new_zero_mask) = parse_mask(&line["mask = ".len()..]);
            one_mask = new_one_mask;
            zero_mask = new_zero_mask;
            println!("Mask: {:b}, {:b}", one_mask, zero_mask);
        } else {
            println!("{}", line);
            let captures = assignment_regex.captures(&line).unwrap();
            let position: u64 = captures[1].parse().unwrap();
            let value: u64 = captures[2].parse().unwrap();
            let value = (value | one_mask) & zero_mask;
            println!("{} at {}", value, position);
            memory.insert(position, value);
        }
    }

    let sum: u64 = memory.iter().map(|(_, val)| val).sum();
    println!("Sum: {}", sum);
}

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut one_mask = 0;
    let mut zero_mask = 0xFFFFFFFFF;
    for i in 0..36 {
        let str_i = 35 - i;
        if mask.chars().nth(str_i).unwrap() == '1' {
            one_mask |= (1 as u64) << i;
        } else if mask.chars().nth(str_i).unwrap() == '0' {
            zero_mask ^= (1 as u64) << i; 
        }
    }

    (one_mask, zero_mask)
}
