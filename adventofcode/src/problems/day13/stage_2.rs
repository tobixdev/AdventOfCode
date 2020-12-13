use crate::util::read_lines;

pub fn run() {
    let busses: Vec<Bus> = read_lines("./src/problems/day13/input.txt").unwrap().nth(1).unwrap().unwrap()
        .split(",")
        .map(|bus| if bus == "x" { Bus::X } else { Bus::Dep(bus.parse().unwrap()) })
        .collect();
    
    let mut expected_time = 0;

    let mut time_inc = 0;
    let mut start_time = 0;
    let mut i = 0;
    for bus in &busses {
        match bus {
            Bus::X => {},
            Bus::Dep(x) => {
                if time_inc < *x  {
                    time_inc = *x;
                    start_time = i;
                }
            }
        };
        i += 1;
    }
    let mut expected_time = 0;

    println!("Starting at {} with inc {}.", start_time, time_inc);

    loop {
        expected_time += time_inc;

        if is_ok_seq(&busses, expected_time - start_time) {
            println!("Expected time: {}", expected_time);
            return;
        }

        if expected_time < 1000 {
            println!("Overflow: {}", expected_time);
        }
    }
}

fn is_ok_seq(busses: &Vec<Bus>, expected_time: u64) -> bool {
    let mut expected_time = expected_time;
    for bus in busses {
        let is_ok = match bus {
            Bus::X => true,
            Bus::Dep(x) => expected_time % *x == 0
        };
        if !is_ok {
            return false;
        }
        expected_time += 1;
    }
    true
}

enum Bus {
    X,
    Dep(u64)
}
