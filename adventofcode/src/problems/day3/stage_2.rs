use super::slope;

pub fn run() {
    let mut results = Vec::<usize>::new();
    results.push(test_slope(1, 1));
    results.push(test_slope(3, 1));
    results.push(test_slope(5, 1));
    results.push(test_slope(7, 1));
    results.push(test_slope(1, 2));
    let result = results.iter().fold(1, |a, b| a * b);
    println!("Result: {}", result);
}

pub fn test_slope(inc_right: usize, inc_down: usize) -> usize {
    let slope = slope::read_slope();
    let mut i = 0;
    let mut j = 0;
    let mut tree_count = 0;
    while i < slope.len() {
        if slope[i][j] == 1 {
            tree_count += 1;
        }
        i += inc_down;
        j += inc_right;
    }

    println!("Tree count {} for ({},{}).", tree_count, inc_right, inc_down);
    return tree_count;
}