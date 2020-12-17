pub fn calc_id(line: &str) -> u32 {
    let row = calc_row(&line);
    let seat = calc_seat(&line);
    row * 8 + seat
}

fn calc_row(line: &str) -> u32 {
    let mut min = 0;
    let mut max = 127;
    for i in 0..7 {
        let character = line.chars().nth(i).unwrap();
        if character == 'B' {
            min = (min + max) / 2 + 1;
        } else if character == 'F' {
            max = (min + max) / 2;
        } else {
            panic!("Unexpected char");
        }
    }
    return min;
}

fn calc_seat(line: &str) -> u32 {
    let mut min = 0;
    let mut max = 7;
    for i in 7..10 {
        let character = line.chars().nth(i).unwrap();
        if character == 'R' {
            min = (min + max) / 2 + 1;
        } else if character == 'L' {
            max = (min + max) / 2;
        } else {
            panic!("Uexpected char");
        }
    }
    return min;
}