
use crate::util::read_lines;

pub fn run() {
    let mut adapters: Vec<u32> = read_lines("./src/problems/day10/input.txt").unwrap().map(|line| line.unwrap().parse::<u32>().unwrap()).collect();
    adapters.sort();
    let mut last_adapter = 0;
    let mut count_1_diff = 0;
    let mut count_3_diff = 0;
    for adapter in adapters {
        let diff = adapter - last_adapter;
        if diff == 1 {
            count_1_diff += 1;
        } else if diff == 3 {
            count_3_diff += 1;
        }
        last_adapter = adapter;
    }
    count_3_diff += 1; // difference to device
    println!("{} * {} = {}", count_1_diff, count_3_diff, count_1_diff * count_3_diff);
}