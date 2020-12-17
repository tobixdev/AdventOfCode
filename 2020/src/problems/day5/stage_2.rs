use crate::problems::day5::boarding_pass::calc_id;

use crate::util::read_lines;
pub fn run() {
    let lines = read_lines("./src/problems/day5/input.txt").unwrap();

    let mut ids = Vec::<u32>::new();

    for line in lines {
        let line = line.unwrap();
        let id = calc_id(&line);
        ids.push(id);
    }

    ids.sort();

    let mut last_id = ids[0];
    for id in ids.iter().skip(1) {
        if *id != last_id + 1 {
            assert!(*id == last_id + 2);
            println!("Found gap {}", id - 1);
        }
        last_id = *id;
    }
}
