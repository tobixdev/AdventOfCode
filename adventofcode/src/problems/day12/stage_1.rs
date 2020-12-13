use crate::util::read_lines;

#[derive(Clone, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
    Left,
    Front,
    Right,
}

struct State {
    direction: Direction,
    pos: (i32, i32),
}

impl State {
    fn new() -> State {
        State {
            direction: Direction::East,
            pos: (0, 0),
        }
    }

    fn apply(&mut self, dir: Direction, value: i32) {
        match dir {
            Direction::Front => {
                self.move_ship(self.direction.clone(), value);
            }
            Direction::Left => {
                self.rleft(value);
            }
            Direction::Right => {
                self.rright(value);
            }
            d => {
                self.move_ship(d, value);
            }
        };
    }
    
    fn move_ship(&mut self, dir: Direction, value: i32) {
        self.pos = match dir {
            Direction::East => (self.pos.0 + value, self.pos.1),
            Direction::West => (self.pos.0 - value, self.pos.1),
            Direction::South => (self.pos.0, self.pos.1 - value),
            Direction::North => (self.pos.0, self.pos.1 + value),
            _ => panic!("Unknown move"),
        }
    }

    fn rleft(&mut self, value: i32) {
        let mut value = value;
        while value > 0 {
            self.direction = match self.direction {
                Direction::East => Direction::North,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::North => Direction::West,
                _ => panic!("Unexpected")
            };
            value -= 90;
        }
    }

    fn rright(&mut self, value: i32) {
        let mut value = value;
        while value > 0 {
            self.direction = match self.direction {
                Direction::East => Direction::South,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                Direction::North => Direction::East,
                _ => panic!("Unexpected")
            };
            value -= 90;
        }
    }
}

pub fn run() {
    let instructions = read_lines("./src/problems/day12/input.txt")
        .unwrap()
        .map(|l| parse_line(&l.unwrap()));

    let mut state = State::new();

    for (dir, value) in instructions {
        state.apply(dir, value);
    }

    println!("{},{}", state.pos.0, state.pos.1);
}

fn parse_line(line: &str) -> (Direction, i32) {
    let direction = match line.chars().nth(0).unwrap() {
        'N' => Direction::North,
        'W' => Direction::West,
        'S' => Direction::South,
        'E' => Direction::East,
        'L' => Direction::Left,
        'F' => Direction::Front,
        'R' => Direction::Right,
        _ => panic!("Wrong direction"),
    };
    (direction, line[1..].parse().unwrap())
}
