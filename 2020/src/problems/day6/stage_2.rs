use crate::util::read_lines;

pub fn run() {
    let lines = read_lines("./src/problems/day6/input.txt").unwrap();
    let mut groups = Vec::<u32>::new();
    let mut group = 0x3FFFFFF;
    for line in lines {
        let line = line.unwrap();

        if line == "" {
            groups.push(group);
            group = 0x3FFFFFF;
        } else {
            let mut person = 0;
            for character in line.chars() {
                let index = character as u32 - 'a' as u32; 
                person |= 1 << index;
            }
            group &= person;
        }
    }   
    groups.push(group);

    let sum: u32 = groups.iter().map(|group| count_ones(*group)).sum();
    println!("Count: {}", sum);
}

fn count_ones(group: u32) -> u32 {
    (0..32).map(|i| (group >> i) & 1).sum()
}