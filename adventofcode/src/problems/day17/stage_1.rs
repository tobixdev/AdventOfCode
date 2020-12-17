use crate::util::read_lines;
use core::hash::Hash;
use core::ops::Add;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let lines = read_lines("./src/problems/day17/input.txt").unwrap();
    let mut energy_source = EnergySource::new();

    let mut i = 0;
    for line in lines {
        let line = line.unwrap();
        let mut j = 0;
        for character in line.chars() {
            if character == '#' {
                energy_source.activate(Point::new(i, j, 0));
            }
            j += 1;
        }
        i += 1;
    }

    energy_source.next_state();
    energy_source.next_state();
    energy_source.next_state();
    energy_source.next_state();
    energy_source.next_state();
    energy_source.next_state();

    println!("{}", energy_source.len());
}

pub struct EnergySource {
    active: HashSet<Point>,
}

impl EnergySource {
    pub fn new() -> EnergySource {
        EnergySource {
            active: HashSet::new(),
        }
    }

    pub fn activate(&mut self, point: Point) {
        self.active.insert(point);
    }

    pub fn next_state(&mut self) {
        let mut counts = HashMap::new();

        for active in self.active.iter() {
            for neighbourgh in Point::neighbourghs().iter() {
                let point = active + neighbourgh;
                if counts.contains_key(&point) {
                    let count = counts[&point];
                    counts.insert(point, count + 1);
                } else {
                    counts.insert(point, 1);
                }
            }
        }

        self.active = counts.iter()
            .filter(|(key, &value)| {
                if self.active.contains(&key) {
                    return value == 2 || value == 3;
                } else {
                    return value == 3;
                }
            })
            .map(|(&key, _)| key)
            .collect();
    }

    pub fn len(&self) -> usize {
        self.active.len()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, right: &'b Point) -> Point {
        Point::new(self.x + right.x, self.y + right.y, self.z + right.z)
    }
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn neighbourghs() -> [Point; 26] {
        [
            Point::new(-1, -1, -1),
            Point::new(-1, -1, 0),
            Point::new(-1, -1, 1),
            Point::new(-1, 0, -1),
            Point::new(-1, 0, 0),
            Point::new(-1, 0, 1),
            Point::new(-1, 1, -1),
            Point::new(-1, 1, 0),
            Point::new(-1, 1, 1),
            Point::new(0, -1, -1),
            Point::new(0, -1, 0),
            Point::new(0, -1, 1),
            Point::new(0, 0, -1),
            Point::new(0, 0, 1),
            Point::new(0, 1, -1),
            Point::new(0, 1, 0),
            Point::new(0, 1, 1),
            Point::new(1, -1, -1),
            Point::new(1, -1, 0),
            Point::new(1, -1, 1),
            Point::new(1, 0, -1),
            Point::new(1, 0, 0),
            Point::new(1, 0, 1),
            Point::new(1, 1, -1),
            Point::new(1, 1, 0),
            Point::new(1, 1, 1),
        ]
    }
}
