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
                ('#', neighbourghs) if neighbourghs >= 5 => 'L',
                (x, _) => x
            }
        }
    }
}

fn count_adjacent(ship: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut acc = 0;
    for i in -1 as i32..=1 {
        for j in -1 as i32..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if sees_occupied_seat(ship, row, col, i, j) {
                acc += 1;
            }
        }
    }
    acc
}

fn sees_occupied_seat(ship: &Vec<Vec<char>>, row: usize, col: usize, offset_row: i32, offset_col: i32) -> bool {
    let mut i: i32 = row as i32 + offset_row;
    let mut j: i32 = col as i32 + offset_col;
    while i >= 0 && i < ship.len() as i32 && j >= 0 && j < ship[0].len() as i32 {
        if ship[i as usize][j as usize] == '#' {
            return true;
        }
        if ship[i as usize][j as usize] == 'L' {
            return false;
        }
        i += offset_row;
        j += offset_col;
    }
    false
}