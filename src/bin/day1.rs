use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_measurements(path: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let measurement = match line.unwrap().parse::<i32>() {
            Ok(val) => val,
            Err(_) => panic!("Invalid value!")
        };
        result.push(measurement);
    }
    return result;
}

fn main() {
    let measurements = read_measurements("data/day1-input");
    let mut increased_count = 0;
    for ind in 1..measurements.len() {
        let diff = measurements[ind] - measurements[ind-1];
        if diff > 0 {
            increased_count += 1;
        }
    }
    println!("Number increased: {}", increased_count);
}
