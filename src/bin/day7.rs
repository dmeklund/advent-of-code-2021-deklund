use std::fs::File;
use std::io::{BufRead, BufReader};

use ndarray::Array1;

struct CrabPositions {
    positions: Array1<i32>,
}

impl CrabPositions {
    fn from_file(path: &str) -> CrabPositions {
        let line = BufReader::new(File::open(path).unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap();
        let positions: Vec<i32> = line
            .split(",")
            .map(|pos| pos.parse::<i32>().unwrap())
            .collect();
        CrabPositions {positions: Array1::from_vec(positions)}
    }

    fn alignment_fuel(&self, position: i32) -> i32 {
        (&self.positions - position)
            .mapv(i32::abs)
            .mapv(|val| val*(val+1)/2)
            .iter()
            .sum()
    }

    fn minimum_alignment_fuel(&self) -> (i32, i32) {
        let min_pos = *self.positions.iter().min().unwrap();
        let max_pos = *self.positions.iter().max().unwrap();
        let mut best_pos = -1;
        let mut least_fuel = -1;
        for pos in min_pos..max_pos+1 {
            let fuel = self.alignment_fuel(pos);
            if fuel < least_fuel || least_fuel == -1 {
                least_fuel = fuel;
                best_pos = pos;
            }
        }
        (best_pos, least_fuel)
    }
}


fn main(){
    let positions = CrabPositions::from_file("data/day7-input");
    let (best_pos, least_fuel) = positions.minimum_alignment_fuel();
    println!("Best position: {} (fuel: {})", best_pos, least_fuel);
}