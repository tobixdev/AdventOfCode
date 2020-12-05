use crate::problems::day5::boarding_pass::calc_id;

use crate::util::read_lines;
pub fn run() {
    let lines = read_lines("./src/problems/day5/input.txt").unwrap();
    let mut max_boarding_pass_id = 0;
    for line in lines {
        let line = line.unwrap();
        let id = calc_id(&line);
        if id > max_boarding_pass_id {
            max_boarding_pass_id = id;
        }
    }
    println!("Maximum: {}", max_boarding_pass_id);
}