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

fn count_increased(measurements: &Vec<i32>) -> i32 {
    let mut increased_count = 0;
    for ind in 1..measurements.len() {
        let diff = measurements[ind] - measurements[ind-1];
        if diff > 0 {
            increased_count += 1;
        }
    }
    println!("Number increased: {}", increased_count);
    increased_count
}

fn count_window_increased(measurements: &Vec<i32>) -> usize {
    let windowed: Vec<i32> = (2..measurements.len()).map(|idx| measurements[idx-2..idx+1].iter().sum()).collect();
    println!("windowed: {:?}", windowed);
    let increased = (1..windowed.len()).filter(|idx| windowed[*idx] > windowed[*idx-1]).count();
    println!("Number increased: {:?}", increased);
    increased
}


fn main() {
    let measurements = read_measurements("data/day1-input");
    let test_measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    println!("Test measurements");
    count_increased(&test_measurements);
    count_window_increased(&test_measurements);
    println!("Real measurements");
    count_increased(&measurements);
    count_window_increased(&measurements);
}
