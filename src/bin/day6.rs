use std::fs::File;
use std::io::{BufRead, BufReader};

struct LanternFish {
    counter: i32,
}

impl LanternFish {
    fn simulate_day(&mut self) -> Option<LanternFish> {
        if self.counter == 0 {
            self.counter = 6;
            return Some(LanternFish { counter: 8 });
        } else {
            self.counter -= 1;
            return None;
        }
    }
}

struct LanternFishSet {
    fish: Vec<LanternFish>,
}

impl LanternFishSet {
    fn from_file(path: &str) -> LanternFishSet {
        let line = BufReader::new(File::open(path).unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap();
        let fish: Vec<LanternFish> = line
            .split(",")
            .map(|count| count.parse::<i32>().unwrap())
            .map(|counter| LanternFish{counter})
            .collect();
        LanternFishSet { fish }
    }

    fn simulate_day(&mut self) {
        let mut newfish = Vec::new();
        for fish in &mut self.fish {
            match fish.simulate_day() {
                Some(f) => newfish.push(f),
                None => (),
            }
        }
        self.fish.append(&mut newfish);
    }

    fn count(&self) -> usize {
        self.fish.len()
    }
}


struct BigLanternFishSet {
    fish: [i64; 9],
}

impl BigLanternFishSet {
    fn from_file(path: &str) -> BigLanternFishSet {
        let fishset = LanternFishSet::from_file(path);
        let mut fisharray = [0; 9];
        for f in fishset.fish {
            fisharray[f.counter as usize] += 1;
        }
        BigLanternFishSet { fish: fisharray }
    }

    fn simulate_day(&mut self) {
        let mut newarray = [0; 9];
        for ind in 1..9 {
            newarray[ind-1] = self.fish[ind];
        }
        newarray[6] += self.fish[0];
        newarray[8] += self.fish[0];
        self.fish = newarray;
    }

    fn count(&self) -> i64 {
        self.fish.into_iter().sum::<i64>()
    }
}

fn main() {
    let mut fishset = BigLanternFishSet::from_file("data/day6-input");
    for day in 0..256 {
        fishset.simulate_day();
        println!("After {} days, there are {} fish", day+1, fishset.count());
    }
}
