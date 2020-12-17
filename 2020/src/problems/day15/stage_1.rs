use std::collections::HashMap;
use crate::util::read_lines;

pub fn run() {
    let mut last_occured: HashMap<u32, Vec<u32>> = HashMap::new();
    let line = read_lines("./src/problems/day15/input.txt").unwrap().nth(0).unwrap().unwrap();
    let mut turn = 1;
    let mut last_number = 0;
    for num in line.split(",") {
        last_number = num.parse().unwrap();
        println!("turn {}: {}", turn, last_number);
        last_occured.insert(last_number, vec!(turn));
        turn += 1;
    }

    while turn <= 2020 {
        let new_vec = Vec::new();
        let occurences = if last_occured.contains_key(&last_number) { last_occured.get(&last_number).unwrap() } else { &new_vec };
        if occurences.len() > 1 {
            let diff = occurences[occurences.len() - 1] - occurences[occurences.len() - 2];
            println!("turn {}: {}", turn, diff);
            if last_occured.contains_key(&diff) {
                last_occured.get_mut(&diff).unwrap().push(turn);
            } else {
                last_occured.insert(diff, vec!(turn));
            }
            last_number = diff;
        } else {
            println!("turn {}: 0", turn);
            if last_occured.contains_key(&0) {
                last_occured.get_mut(&0).unwrap().push(turn);
            } else {
                last_occured.insert(0, vec!(turn));
            }
            last_number = 0;
        }
        turn += 1;
    }
}