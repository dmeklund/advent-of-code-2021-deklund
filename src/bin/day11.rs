use ndarray::Array2;
use std::io::{BufRead, BufReader};
use std::fs::File;

struct OctopusMap {
    energies: Array2<i32>,
}

impl OctopusMap {
    fn from_file(path: &str) -> OctopusMap {
        let lines: Vec<String> = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(Result::unwrap)
            .collect();
        let nrows = lines.len();
        let ncols = lines[0].len();
        let mut energies = Array2::zeros((nrows, ncols));
        for (row, line) in lines.iter().enumerate() {
            for (col, character) in line.chars().enumerate() {
                let energy = character.to_digit(10).unwrap() as i32;
                energies[(row, col)] = energy;
            }
        }
        OctopusMap { energies }
    }

    fn step(&mut self) -> i32 {
        self.energies += 1;
        let mut flashed: Array2<i32> = Array2::zeros((self.energies.nrows(), self.energies.ncols()));
        loop {
            let indices: Vec<(usize, usize)> = self.energies
                .indexed_iter()
                .filter(|(index, &_energy)| flashed[(index.0, index.1)] == 0)
                .filter(|(_index, &energy)| energy >= 10)
                .map(|(index, _energy)| index)
                .collect();
            if indices.len() == 0 {
                break;
            }
            for index in indices {
                self.flash_at(index);
                flashed[index] = 1;
            }
        }
        let mut flashes = 0;
        self.energies.map_inplace(|valref| {
            if *valref >= 10 {
                flashes += 1;
                *valref = 0;
            }
        });
        flashes
    }

    fn flash_at(&mut self, (row, col): (usize, usize)) {
        self.energies[(row, col)] += 1;
        if let Some(valref) = self.energies.get_mut((row+1, col)) {
            *valref += 1;
        }
        if let Some(valref) = self.energies.get_mut((row, col+1)) {
            *valref += 1;
        }
        if let Some(valref) = self.energies.get_mut((row+1, col+1)) {
            *valref += 1;
        }
        if let Some(newrow) = row.checked_sub(1) {
            if let Some(valref) = self.energies.get_mut((newrow, col)) {
                *valref += 1;
            }
            if let Some(valref) = self.energies.get_mut((newrow, col+1)) {
                *valref += 1;
            }
            if let Some(newcol) = col.checked_sub(1) {
                if let Some(valref) = self.energies.get_mut((newrow, newcol)) {
                    *valref += 1;
                }
            }
        }
        if let Some(newcol) = col.checked_sub(1) {
            if let Some(valref) = self.energies.get_mut((row, newcol)) {
                *valref += 1;
            }
            if let Some(valref) = self.energies.get_mut((row+1, newcol)) {
                *valref += 1;
            }
        }
    }
}

fn main() {
    let mut octopusmap = OctopusMap::from_file("data/day11-input");
    let mut num_flashes = 0;
    println!("Before any steps: {:?} ({})", octopusmap.energies, octopusmap.energies.len());
    let mut step = 0;
    loop {
        num_flashes += octopusmap.step();
        step += 1;
        // println!("After step {} ({} flashes)", step+1, num_flashes);
        // println!("{:?}", octopusmap.energies);
        if octopusmap.energies.iter().filter(|&&val| val == 0).count() == octopusmap.energies.len() {
            println!("All flash after step {}", step);
            break;
        }
    }
    println!("Total flashes: {}", num_flashes);
}
