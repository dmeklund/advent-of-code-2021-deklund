use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Submarine {
    depth: i32,
    horiz: i32
}

impl Submarine {
    pub fn new() -> Self {
        Submarine { depth: 0, horiz: 0 }
    }

    pub fn forward(&mut self, units: i32) {
        self.horiz += units;
    }

    pub fn down(&mut self, units: i32) {
        self.depth += units;
    }

    pub fn up(&mut self, units: i32) {
        self.depth -= units;
    }

    pub fn run_command(&mut self, cmd: &str) {
        match cmd {
            cmd if cmd.starts_with("forward") => self.forward(cmd["forward".len()+1..].parse::<i32>().unwrap()),
            cmd if cmd.starts_with("down") => self.down(cmd["down".len()+1..].parse::<i32>().unwrap()),
            cmd if cmd.starts_with("up") => self.up(cmd["up".len()+1..].parse::<i32>().unwrap()),
            _ => panic!("Help!")
        }
    }

    pub fn run_from_file(&mut self, path: &str) {
        for line in BufReader::new(File::open(path).unwrap()).lines() {
            let command = line.unwrap();
            self.run_command(&command);
        }
    }

    pub fn horiz(&self) -> i32 { self.horiz }
    pub fn depth(&self) -> i32 { self.depth }
}

fn main() {
    let mut mysub = Submarine::new();
    mysub.down(5);
    mysub.forward(3);
    println!("sub test: {:?}", mysub);
    mysub = Submarine::new();
    mysub.run_from_file("data/day2-input");
    println!("sub result: {:?} ({})", mysub, mysub.horiz() * mysub.depth());
}
