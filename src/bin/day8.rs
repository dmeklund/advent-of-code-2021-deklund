use std::fs::File;
use std::io::{BufRead, BufReader};

struct SignalPatternDisplay {
    patterns: Vec<String>,
    values: Vec<String>,
}

impl SignalPatternDisplay {
    fn from_file(path: &str) -> Vec<SignalPatternDisplay> {
        let mut results = vec![];
        for line in BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|line| line.unwrap()) {
            let mut split = line.split(" | ");
            let patterns: Vec<String> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect();
            let values: Vec<String> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect();
            results.push(SignalPatternDisplay { patterns, values });
        }
        results
    }
}


fn main() {
    let alldisplays = SignalPatternDisplay::from_file("data/day8-input");
    // let alldisplays = SignalPatternDisplay::from_file("data/day8-sample");
    let mut count = 0;
    for display in alldisplays {
        for value in display.values {
            let length = value.len();
            if length == 2 || length == 4 || length == 3 || length == 7 {
                println!("Found pattern {}", value);
                count += 1;
            }
        }
    }
    println!("Total count: {}", count);
}