use std::ops::{Index, IndexMut};

use crate::util::read_lines;

pub struct Slope {
    slope: [SlopeLine; 323],
}

impl Slope {
    pub fn len(&self) -> usize {
        return self.slope.len();
    }
}

impl Index<usize> for Slope {
    type Output = SlopeLine;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.slope[index];
    }
}

impl IndexMut<usize> for Slope {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.slope[index];
    }
}

#[derive(Copy, Clone)]
pub struct SlopeLine {
    line: [u8; 31],
}

impl Index<usize> for SlopeLine {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.line[index % self.line.len()];
    }
}

impl IndexMut<usize> for SlopeLine {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.line[index % self.line.len()];
    }
}

pub fn read_slope() -> Slope {
    let lines = read_lines("./src/problems/day3/slope.txt").unwrap();

    let mut slope: [SlopeLine; 323] = [SlopeLine {
        line: [0 as u8; 31],
    }; 323];

    let mut i: usize = 0;
    for line in lines {
        let mut j: usize = 0;
        for char in line.unwrap().chars() {
            slope[i][j] = if char == '.' { 0 } else { 1 };
            j += 1;
        }
        i += 1;
    }

    return Slope { slope: slope };
}
