use crate::util::read_lines;

pub fn run() {
    let busses: Vec<Bus> = read_lines("./src/problems/day13/input.txt").unwrap().nth(1).unwrap().unwrap()
        .split(",")
        .map(|bus| if bus == "x" { Bus::X } else { Bus::Dep(bus.parse().unwrap()) })
        .collect();
    
    let mut time_inc = 1;
    let mut t = 0;
    for bus in &busses {
        match bus {
            Bus::X => {},
            Bus::Dep(x) => {
                while t % x != 0 {
                    t += time_inc;
                }
                time_inc = time_inc * x;
            }
        };
        t += 1;
    }
    println!("Result: {}", t as usize - busses.len());
}

enum Bus {
    X,
    Dep(u64)
}

impl Bus {
    fn val(&self) -> u64 {
        match self {
            Bus::X => 0,
            Bus::Dep(x) => *x
        }
    }
}