use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_diagnostics(path: &str) -> Vec<String> {
    let mut diagnostics = Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        diagnostics.push(line.unwrap());
    }
    diagnostics
}

fn compute_counts(diagnostics: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut zerocount = Vec::new();
    let mut onecount = Vec::new();
    for number in diagnostics {
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
    (zerocount, onecount)
}

fn compute_power(zerocount: &Vec<u32>, onecount: &Vec<u32>) -> u32 {
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

fn compute_oxygen(diagnostics: &Vec<String>) -> u32 {
    let mut filtered = diagnostics.clone();
    for ind in 0..diagnostics[0].len() {
        let (zeros, ones) = compute_counts(&filtered);
        if zeros[ind] > ones[ind] {
            filtered = filtered.into_iter().filter(|val| { val.as_bytes()[ind] == b'0' }).collect();
        } else {
            println!("Grabbing ones for index {}", ind);
            filtered = filtered.into_iter().filter(|val| { val.as_bytes()[ind] == b'1' }).collect();
        }
        println!("(Oxygen) filtered to {} values", filtered.len());
        if filtered.len() == 1 {
            println!("Final oxygen value: {}", filtered[0]);
            break;
        }
    }
    if filtered.len() != 1 {
        panic!("Couldn't compute oxygen!");
    }
    u32::from_str_radix(filtered[0].as_str(), 2).unwrap()
}

fn compute_co2(diagnostics: &Vec<String>) -> u32 {
    let mut filtered = diagnostics.clone();
    for ind in 0..diagnostics[0].len() {
        let (zeros, ones) = compute_counts(&filtered);
        if zeros[ind] > ones[ind] {
            filtered = filtered.into_iter().filter(|val| { val.as_bytes()[ind] == b'1' }).collect();
        } else {
            println!("Grabbing ones for index {}", ind);
            filtered = filtered.into_iter().filter(|val| { val.as_bytes()[ind] == b'0' }).collect();
        }
        println!("(Oxygen) filtered to {} values", filtered.len());
        if filtered.len() == 1 {
            println!("Final oxygen value: {}", filtered[0]);
            break;
        }
    }
    if filtered.len() != 1 {
        panic!("Couldn't compute oxygen!");
    }
    u32::from_str_radix(filtered[0].as_str(), 2).unwrap()
}

fn main() {
    let diagnostics = read_diagnostics("data/day3-input");
    let (zerocount, onecount) = compute_counts(&diagnostics);
    let power = compute_power(&zerocount, &onecount);
    println!("Power: {}", power);
    let oxygen = compute_oxygen(&diagnostics);
    let co2 = compute_co2(&diagnostics);
    println!("Oxygen: {}, CO2: {}, Life support: {}", oxygen, co2, oxygen*co2);
}
