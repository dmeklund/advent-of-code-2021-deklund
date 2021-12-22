use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ndarray::{Array2, NdIndex};

struct HeightMap {
    heights: Array2<i32>,
}

struct IndexIterator {
    nrows: usize,
    ncols: usize,
    row: usize,
    col: usize,
}

struct Basins {
    basin_map: Array2<usize>,
    num_basins: usize,
}

impl Basins {
    fn count(&self) -> usize {
        *self.basin_map.iter().max().unwrap()
    }

    fn sizes(&self) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        for basin_num in 1..self.num_basins+1 {
            result.insert(
                basin_num,
                self.basin_map
                    .iter()
                    .filter(|&&basin| basin == basin_num)
                    .count()
            );
        }
        result
    }
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

    fn mark_basin(
            &self,
            basin_map: &mut Array2<usize>,
            (row, col): (usize, usize),
            basin_num: usize)
    {
        if basin_map[(row, col)] == basin_num {
            return;
        }
        basin_map[(row, col)] = basin_num;
        let indices: Vec<(usize, usize)> = [
            (Some(row+1), Some(col)),
            (row.checked_sub(1), Some(col)),
            (Some(row), Some(col+1)),
            (Some(row), col.checked_sub(1)),
        ]
            .iter()
            .filter(|(row, col)| row.is_some() && col.is_some())
            .map(|(row, col)| (row.unwrap(), col.unwrap()))
            .collect();
        for index in indices {
            match self.heights.get(index) {
                Some(&height) if height != 9 => self.mark_basin(basin_map, index, basin_num),
                _ => ()
            }
        }

    }

    fn basins(&self) -> Basins {
        let mut basin_map = Array2::zeros((self.heights.nrows(), self.heights.ncols()));
        let mut basin_count = 0;
        for ((row, col), &value) in self.heights.indexed_iter() {
            if value == 9 {
                continue;
            }
            let mut basin_num = basin_map[(row, col)];
            if basin_num == 0 {
                basin_count += 1;
                basin_num = basin_count;
                self.mark_basin(&mut basin_map, (row, col), basin_num);
            }
        }
        Basins { basin_map, num_basins: basin_count }
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
    let basins = map.basins();
    let mut sizes: Vec<(usize, usize)> = basins.sizes().into_iter().collect();
    sizes.sort_by(|&(anum, asize), &(bnum, bsize)| asize.cmp(&bsize));
    println!("Found {} basins", basins.count());
    println!("{:?}", basins.basin_map);
    println!(
        "Three biggest: {}, {}, {} ({})",
        sizes[sizes.len()-3].1,
        sizes[sizes.len()-2].1,
        sizes[sizes.len()-1].1,
        sizes[sizes.len()-3].1 * sizes[sizes.len()-2].1 * sizes[sizes.len()-1].1
    )
}