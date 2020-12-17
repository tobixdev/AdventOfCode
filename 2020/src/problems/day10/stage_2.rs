
use crate::util::{powerset, read_lines};

pub fn run() {
    let mut adapters: Vec<u32> = read_lines("./src/problems/day10/input.txt").unwrap().map(|line| line.unwrap().parse::<u32>().unwrap()).collect();
    adapters.sort();
    
    // We have to locate the required numbers (e.g. 4 -> (7) -> 10). These numbers split the problem into multiple independent problems. The smaller
    // independent problems can be solved using a brute force method. Then multiply the solutions of the subproblems to get the solution to the
    // actual problem.
    let mut chunks:Vec<Vec<u32>> = Vec::new();
    let mut chunk = Vec::new();
    chunk.push(0);
    for i in 0..(adapters.len()-1)  {
        if adapters[i + 1] - chunk[chunk.len() - 1] > 3 {
            chunk.push(adapters[i]);
            chunks.push(chunk);
            chunk = Vec::new();
            chunk.push(adapters[i]);
        } else {
            chunk.push(adapters[i]);
        }
    }
    let last_adapter = adapters[adapters.len() - 1];
    chunk.push(last_adapter);
    chunks.push(chunk);

    let mut arrangements: u64 = 1;
    for mut chunk in chunks {
        print!("{:?}", chunk);
        let chunk_arrangements = count_chunk_arrangements(&mut chunk);
        println!(" ({})", chunk_arrangements);
        arrangements *= chunk_arrangements;
    }

    println!("Possible arrangements: {}", arrangements);
}

fn count_chunk_arrangements(chunk: &mut Vec<u32>) -> u64 {
    let mut chunk_arrangements = 0;
    let fixed_lower = chunk[0];
    let fixed_upper = chunk[chunk.len() - 1];
    chunk.remove(0);
    chunk.remove(chunk.len() - 1);
    for mut candidate in powerset(&chunk) {
        candidate.insert(0, fixed_lower);
        candidate.push(fixed_upper);
        if is_valid(&candidate) {
            chunk_arrangements += 1;
        }
    }
    chunk_arrangements
}

fn is_valid(chunk: &Vec<u32>) -> bool {
    let mut last_elem = chunk[0];
    for elem in chunk {
        if elem - last_elem > 3 {
            return false;
        }
        last_elem = *elem;
    }

    true
}