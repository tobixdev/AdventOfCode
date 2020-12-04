use super::slope;

pub fn run() {
    let slope = slope::read_slope();
    let mut i = 0;
    let mut j = 0;
    let mut tree_count = 0;
    while i < slope.len() {
        if slope[i][j] == 1 {
            tree_count += 1;
        }
        i += 1;
        j += 3;
    }

    println!("Tree count {}.", tree_count);
}