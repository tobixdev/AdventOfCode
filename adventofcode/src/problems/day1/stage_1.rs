use crate::util::read_lines;

pub fn run() {
    let lines = read_lines("./src/problems/day1/input.txt").unwrap();
    let entries: Vec<u32> = lines.into_iter()
        .map(|entry| entry.unwrap().parse::<u32>().unwrap())
        .collect();

    for i in &entries {
        for j in &entries {
            if i + j == 2020 {
                println!("{} * {} = {}", i, j, i * j);
                return;
            }
        }
    }
}