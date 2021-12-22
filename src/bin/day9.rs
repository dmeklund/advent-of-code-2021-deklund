use std::fs::File;
use std::io::{BufRead, BufReader};

use ndarray::Array2;

struct HeightMap {
    heights: Array2<i32>,
}

struct IndexIterator {
    nrows: usize,
    ncols: usize,
    row: usize,
    col: usize,
}

impl Iterator for IndexIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.row == self.nrows {
            None
        } else {
            let result = (self.row, self.col);
            self.col += 1;
            if self.col == self.ncols {
                self.row += 1;
                self.col = 0;
            }
            Some(result)
        }
    }
}

impl HeightMap {
    fn from_file(path: &str) -> HeightMap {
        let lines: Vec<String> = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(Result::unwrap)
            .collect();
        let nrows = lines.len();
        let ncols = lines[0].len();
        let mut heights = Array2::zeros((nrows, ncols));
        for (row, line) in lines.iter().enumerate() {
            for (col, character) in line.chars().enumerate() {
                let height = character.to_digit(10).unwrap() as i32;
                heights[(row, col)] = height;
            }
        }
        HeightMap { heights }
    }

    fn is_low_point(&self, (row, col): (usize, usize)) -> bool {
        let val = self.heights[(row, col)];
        (row == 0 || self.heights[(row-1, col)] > val) &&
            (col == 0 || self.heights[(row, col-1)] > val) &&
            (row == self.heights.nrows()-1 || self.heights[(row+1, col)] > val) &&
            (col == self.heights.ncols()-1 || self.heights[(row, col+1)] > val)
    }

    fn indices(&self) -> IndexIterator {
        IndexIterator {
            nrows: self.heights.nrows(),
            ncols: self.heights.ncols(),
            row: 0,
            col: 0,
        }
    }

    fn risk_level(&self, index: (usize, usize)) -> i32 {
        1 + self.heights[index]
    }
}

fn main() {
    let map = HeightMap::from_file("data/day9-input");
    let lowpoints: Vec<(usize, usize)> = map
        .indices()
        .filter(|&ind| map.is_low_point(ind))
        .collect();
    println!(
        "Sum of risk levels: {}",
        lowpoints
            .iter()
            .map(|&index| map.risk_level(index))
            .sum::<i32>()
    );
}