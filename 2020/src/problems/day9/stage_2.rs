use std::collections::VecDeque;
use crate::util::read_lines;

pub fn run() {
    let target = 1398413738;
    let mut segment: VecDeque<u64> = VecDeque::new();
    for line in read_lines("./src/problems/day9/input.txt").unwrap() {
        let number = line.unwrap().parse().unwrap();
        segment.push_back(number);
        while segment.iter().sum::<u64>() > target {
            segment.pop_front();
        }
        if segment.iter().sum::<u64>() == target {
            println!("Found solution {}.", segment.iter().min().unwrap() + segment.iter().max().unwrap());
            return;
        }
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