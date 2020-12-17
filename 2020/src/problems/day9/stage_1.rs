
use std::collections::VecDeque;
use crate::util::read_lines;

pub fn run() {
    let preamble_length = 25;
    let lines = read_lines("./src/problems/day9/input.txt").unwrap();
    let mut scope: VecDeque<u64> = VecDeque::new();
    for line in lines.take(preamble_length) {
        scope.push_back(line.unwrap().parse().unwrap())
    }
    for line in read_lines("./src/problems/day9/input.txt").unwrap().skip(preamble_length) {
        let number = line.unwrap().parse().unwrap();
        
        if is_solution(&scope, number) {
            println!("Number {}.", number);
            return;
        }

        scope.push_back(number);
        scope.pop_front();
    }
}

fn is_solution(scope: &VecDeque<u64>, number: u64) -> bool {
    for i in 0..scope.len() {
        for j in i..scope.len() {
            if scope[i] + scope[j] == number {
                return false;
            }
        }
    }
    return true;
}