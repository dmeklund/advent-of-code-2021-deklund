use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(path: &str) -> u32 {
    let mut zerocount = Vec::new();
    let mut onecount = Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let number = line.unwrap();
        for (ind, byte) in number.chars().enumerate() {
            if ind >= zerocount.len() {
                zerocount.push(0);
                onecount.push(0);
            }
            match byte {
                '1' => { onecount[ind] += 1 },
                '0' => { zerocount[ind] += 1 },
                _ => { panic!("Invalid entry: {}", byte) }
            }
        }
    }
    let mut gammastr = String::new();
    let mut epsilonstr = String::new();
    for (ind, (zeros, ones)) in zerocount.iter().zip(onecount.iter()).enumerate() {
        println!("Index {}: {} zeros, {} ones", ind, zeros, ones);
        if zeros > ones {
            gammastr.push('0');
            epsilonstr.push('1');
        } else if ones > zeros {
            gammastr.push('1');
            epsilonstr.push('0');
        } else {
            panic!("Equal number of zeros and ones");
        }
    }
    println!("In binary: {}, {}", gammastr, epsilonstr);
    let gamma = u32::from_str_radix(gammastr.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(epsilonstr.as_str(), 2).unwrap();
    println!("Gamma: {}, Epsilon: {}, Power: {}", gamma, epsilon, gamma * epsilon);
    gamma * epsilon
}

fn main() {
    let result = read_input("data/day3-input");
    println!("Result: {}", result);
}
