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
                let x1 = coordstart.0 as i32;
                let x2 = coordend.0 as i32;
                let y1 = coordstart.1 as i32;
                let y2 = coordend.1 as i32;
                if (x1 - x2).abs() != (y1 - y2).abs() {
                    panic!("Line is not horiz, vert, or diagonal: {} ({}, {}, {}, {})", lineval, x1, x2, y1, y2);
                }
                let dx = (x2 - x1).signum();
                let dy = (y2 - y1).signum();
                for delta in 0..(x1 - x2).abs() + 1 {
                    println!("Marking ({}, {}) for {}", x1+dx*delta, y1+dy*delta, lineval);
                    vents[((x1+delta*dx) as usize, (y1+delta*dy) as usize)] += 1;
                }
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