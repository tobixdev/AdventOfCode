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
    let mut floating: Vec<u8> = Vec::new();
    for line in lines {
        if line.starts_with("mask = ") {
            let (new_one_mask, new_floating) = parse_mask(&line["mask = ".len()..]);
            one_mask = new_one_mask;
            floating = new_floating;
        } else {
            println!("{}", line);
            let captures = assignment_regex.captures(&line).unwrap();
            let position: u64 = captures[1].parse().unwrap();
            let value: u64 = captures[2].parse().unwrap();

            for perm in all_permutations_of_floating(position | one_mask, &floating, 0) {
                memory.insert(perm, value);
            }
        }
    }

    let sum: u64 = memory.iter().map(|(_, val)| val).sum();
    println!("Sum: {}", sum);
}

fn parse_mask(mask: &str) -> (u64, Vec<u8>) {
    let mut one_mask = 0;
    let mut floating: Vec<u8> = Vec::new();
    for i in 0..36 {
        let str_i = 35 - i;
        if mask.chars().nth(str_i).unwrap() == '1' {
            one_mask |= (1 as u64) << i;
        } else if mask.chars().nth(str_i).unwrap() == 'X' {
            floating.push(i as u8);
        }
    }

    (one_mask, floating)
}

fn all_permutations_of_floating(base: u64, floating: &Vec<u8>, i: usize) -> Vec<u64> {
    if i >= floating.len() {
        return vec!(base);
    }

    let mut result = Vec::new();

    result.append(&mut all_permutations_of_floating(base, floating, i + 1));
    result.append(&mut all_permutations_of_floating(base ^ (1 <<  floating[i]), floating, i + 1));

    result
}