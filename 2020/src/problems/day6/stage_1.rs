use crate::util::read_lines;

pub fn run() {
    let lines = read_lines("./src/problems/day6/input.txt").unwrap();
    let mut groups = Vec::<u32>::new();
    let mut group = 0;
    for line in lines {
        let line = line.unwrap();

        if line == "" {
            groups.push(group);
            group = 0;
        } else {
            for character in line.chars() {
                let index = character as u32 - 'a' as u32; 
                let mask = 1 << index;
                group |= mask;
            }
        }
    }   
    groups.push(group);

    let sum: u32 = groups.iter().map(|group| count_ones(*group)).sum();
    println!("Count: {}", sum);
}

fn count_ones(group: u32) -> u32 {
    (0..32).map(|i| (group >> i) & 1).sum()
}