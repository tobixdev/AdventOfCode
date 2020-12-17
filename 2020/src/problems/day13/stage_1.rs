use crate::util::read_lines;

pub fn run() {
    let min_time: u32 = read_lines("./src/problems/day13/input.txt").unwrap().nth(0).unwrap().unwrap().parse().unwrap();
    let busses: Vec<u32> = read_lines("./src/problems/day13/input.txt").unwrap().nth(1).unwrap().unwrap()
        .split(",")
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse().unwrap())
        .collect();
    
    let mut bus_timings: Vec<(u32, u32)> = busses.iter()
        .map(|b| (*b, min_time + b - (min_time % b)))
        .collect();
    bus_timings.sort_by_key(|b1| b1.1);
    let bus = bus_timings.first().unwrap();

    println!("first: {}, {} -> {}", bus.0, bus.1, bus.0 * (bus.1 - min_time));
}
