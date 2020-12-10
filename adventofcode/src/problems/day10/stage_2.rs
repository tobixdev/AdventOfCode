
use crate::util::read_lines;

pub fn run() {
    let mut adapters: Vec<u32> = read_lines("./src/problems/day10/input.txt").unwrap().map(|line| line.unwrap().parse::<u32>().unwrap()).collect();
    adapters.sort();
    let mut last_adapter = 0;
    let mut arrangements: u64 = 1;
    for i in 0..adapters.len() {
        let mut candidates = 1;
        if i < adapters.len() - 1 && adapters[i + 1] - last_adapter <= 3 {
            candidates += 1;
        } else if i < adapters.len() - 2 && adapters[i + 2] - last_adapter <= 3 {
            candidates += 1;
        }
        let possible_arrangements = match candidates {
            1 => 1,
            2 => 3,
            3 => 6,
            _ => unreachable!()
        };
        arrangements *= possible_arrangements;
        last_adapter = adapters[i];
    }
    println!("Possible arrangements: {}", arrangements);
}