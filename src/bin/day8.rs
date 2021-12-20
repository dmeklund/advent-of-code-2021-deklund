use std::collections::{HashMap, HashSet};
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

fn digit_segments() -> HashMap<i32, HashSet<u8>> {
    // 0:      1:      2:      3:      4:
    //  aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....
    //
    // 5:      6:      7:      8:      9:
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg
    let mut digitmap = HashMap::new();
    digitmap.insert(0, HashSet::from([b'a', b'b', b'c', b'e', b'f', b'g']));
    digitmap.insert(1, HashSet::from([b'c', b'f']));
    digitmap.insert(2, HashSet::from([b'a', b'c', b'd', b'e', b'g']));
    digitmap.insert(3, HashSet::from([b'a', b'c', b'd', b'f', b'g']));
    digitmap.insert(4, HashSet::from([b'b', b'c', b'd', b'f']));
    digitmap.insert(5, HashSet::from([b'a', b'b', b'd', b'f', b'g']));
    digitmap.insert(6, HashSet::from([b'a', b'b', b'd', b'e', b'f', b'g']));
    digitmap.insert(7, HashSet::from([b'a', b'c', b'f']));
    digitmap.insert(8, HashSet::from([b'a', b'b', b'c', b'd', b'e', b'f', b'g']));
    digitmap.insert(9, HashSet::from([b'a', b'b', b'c', b'd', b'f', b'g']));
    digitmap
}

fn find_mapping(display: &SignalPatternDisplay) -> HashMap<u8, HashSet<u8>> {
    // let result = HashMap::new();
    let mut options: HashMap<u8, HashSet<u8>> = HashMap::new();
    let digitmap = digit_segments();
    let allsegments = digitmap.get(&8).unwrap();
    for pattern in &display.patterns {
        let pbytes: HashSet<u8> = HashSet::from_iter(Vec::from(pattern.as_bytes()));
        let mut potential_matches = HashSet::new();
        let mut non_matches = allsegments.clone();
        // let potential_matches = digitmap
        //     .values()
        //     .filter(|segments| segments.len() == pattern.len())
        //     .collect();
        for (digit, segments) in &digitmap {
            if pattern.len() == segments.len() {
                potential_matches.extend(segments);
                non_matches = non_matches.difference(segments).copied().collect();
            }
            // if pattern.len() == segments.len() {
            //     for wire in &pbytes {
            //         let newset = options.entry(*wire)
            //             .or_insert(segments.clone())
            //             .intersection(&segments)
            //             .copied()
            //             .collect();
            //         options.insert(*wire, newset);
            //     }
            // }
        }
        for wire in &pbytes {
            let newset: HashSet<u8> = options
                .entry(*wire)
                .or_insert(potential_matches.clone())
                .intersection(&potential_matches)
                .copied()
                .collect::<HashSet<u8>>()
                .difference(&non_matches)
                .copied()
                .collect();
            match options.insert(*wire, newset.clone()) {
                Some(prev) => {
                    if newset.len() != prev.len() {
                        println!("Reduced {:?} to {:?}", prev, newset)
                    }
                }
                None => ()
            }

        }
    }
    options
}


fn main() {
    let alldisplays = SignalPatternDisplay::from_file("data/day8-input");
    // let alldisplays = SignalPatternDisplay::from_file("data/day8-sample");
    let mut count = 0;
    for display in alldisplays {
        let mappings = find_mapping(&display);
        // println!("Mappings: {:?}", mappings);
        for value in &display.values {
            let length = value.len();
            if length == 2 || length == 4 || length == 3 || length == 7 {
                // println!("Found pattern {}", value);
                count += 1;
            }
        }
    }
    println!("Total count: {}", count);
}