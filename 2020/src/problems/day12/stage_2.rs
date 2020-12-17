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
    pos: (i32, i32),
    waypoint: (i32, i32)
}

impl State {
    fn new() -> State {
        State {
            pos: (0, 0),
            waypoint: (10, 1)
        }
    }

    fn apply(&mut self, dir: Direction, value: i32) {
        match dir {
            Direction::Front => {
                self.move_ship(value);
            }
            Direction::Left => {
                self.rleft(value);
            }
            Direction::Right => {
                self.rright(value);
            }
            d => {
                self.move_waypoint(d, value);
            }
        };
    }

    fn move_ship(&mut self, value: i32) {
        self.pos = (self.pos.0 + self.waypoint.0 * value, self.pos.1 + self.waypoint.1 * value);
    }

    fn move_waypoint(&mut self, dir: Direction, value: i32) {
        self.waypoint = match dir {
            Direction::East => (self.waypoint.0 + value, self.waypoint.1),
            Direction::West => (self.waypoint.0 - value, self.waypoint.1),
            Direction::South => (self.waypoint.0, self.waypoint.1 - value),
            Direction::North => (self.waypoint.0, self.waypoint.1 + value),
            _ => panic!("Unknown move"),
        }
    }

    fn rleft(&mut self, value: i32) {
        let newdiff_x = self.waypoint.0 * (value as f32).to_radians().cos() as i32 - self.waypoint.1 * (value as f32).to_radians().sin() as i32;
        let newdiff_y = self.waypoint.1 * (value as f32).to_radians().cos() as i32 + self.waypoint.0 * (value as f32).to_radians().sin() as i32;
        self.waypoint = (newdiff_x, newdiff_y);
    }

    fn rright(&mut self, value: i32) {
        self.rleft(-value);
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
