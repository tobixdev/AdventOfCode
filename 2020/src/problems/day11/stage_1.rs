use crate::util::read_lines;

pub fn run() {
    let mut seats: Vec<Vec<char>> = Vec::new();
    let lines = read_lines("./src/problems/day11/input.txt").unwrap();
    for line in lines {
        seats.push(line.unwrap().chars().collect())
    }

    let mut is_fixpoint = false;
    while !is_fixpoint {
        let mut new_seats = seats.clone();
        mutate_seats(&seats, &mut new_seats);
        is_fixpoint = new_seats == seats;
        seats = new_seats;
    }

    let mut count = 0;
    for row in seats {
        for character in row {
            if character == '#' {
                count += 1;
            }
        }
    }
    println!("{} seats occupied.", count);
}

fn mutate_seats(last: &Vec<Vec<char>>, new: &mut Vec<Vec<char>>) {
    for i in 0..new.len() {
        for j in 0..new[i].len() {
            new[i][j] = match (new[i][j], count_adjacent(last, i, j)) {
                ('L', 0) => '#',
                ('#', neighbourghs) if neighbourghs >= 4 => 'L',
                (x, _) => x
            }
        }
    }
}

fn count_adjacent(ship: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut acc = 0;
    let from_i = if row == 0 { 0 } else { row - 1};
    let to_i = if row == ship.len() - 1 { ship.len() - 1 } else { row + 1 };
    let from_j = if col == 0 { 0 } else { col - 1};
    let to_j = if col == ship[0].len() - 1 { ship[0].len() - 1 } else { col + 1 };
    for i in from_i..=to_i {
        for j in from_j..=to_j {
            if i == row && j == col {
                continue;
            }

            if ship[i][j] == '#' {
                acc += 1;
            }
        }
    }
    acc
}