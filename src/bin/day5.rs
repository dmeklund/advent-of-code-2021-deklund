use std::cmp::{min, max};
use std::fs::File;
use std::io::{BufRead, BufReader};

use ndarray::Array2;

struct FloorMap {
    vents: Array2<i32>,
}

impl FloorMap {
    fn from_file(path: &str, size: usize) -> FloorMap {
        let mut vents = Array2::zeros((size, size));
        for line in BufReader::new(File::open(path).unwrap()).lines() {
            let lineval = line.unwrap();
            let mut coordsiter = lineval.split(" -> ");
            let start = coordsiter.next().unwrap();
            let end = coordsiter.next().unwrap();
            let coordstart = FloorMap::parse_coord(start);
            let coordend = FloorMap::parse_coord(end);
            if coordstart.0 == coordend.0 {
                let xval = coordstart.0;
                let (ymin, ymax) = (min(coordstart.1, coordend.1), max(coordstart.1, coordend.1));
                for yval in ymin..ymax+1 {
                    vents[(xval, yval)] += 1;
                }
            } else if coordstart.1 == coordend.1 {
                let yval = coordstart.1;
                let (xmin, xmax) = (min(coordstart.0, coordend.0), max(coordstart.0, coordend.0));
                for xval in xmin..xmax+1 {
                    vents[(xval, yval)] += 1;
                }
            } else {
                // panic!("Only horiz & vert lines supported! {:?}, {:?} ({})", coordstart, coordend, lineval);
                println!("Skipping non-horz/vert line: {}", lineval);
            }
        }
        FloorMap { vents }
    }

    fn parse_coord(coord: &str) -> (usize, usize) {
        let coords: Vec<usize> = coord.split(",").map(|cstr| cstr.parse::<usize>().unwrap()).collect();
        (coords[0], coords[1])
    }
}

fn main() {
    let map = FloorMap::from_file("data/day5-input", 1000);
    // let map = FloorMap::from_file("data/day5-sample", 10);
    println!("{:?}", map.vents);
    let dangerous_count = map.vents.iter().filter(|val| **val>1).count();
    println!("Dangerous count: {}", dangerous_count);
}