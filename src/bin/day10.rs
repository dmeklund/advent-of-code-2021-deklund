use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::fs::File;


enum Chunk {
    Open(ChunkType),
    Close(ChunkType),
}

#[derive(PartialEq, Eq, Debug)]
enum ChunkType {
    Square,
    Curly,
    Angle,
    Round
}

impl Chunk {
    fn from_char(character: char) -> Chunk {
        if character == '[' {
            Chunk::Open(ChunkType::Square)
        } else if character == ']' {
            Chunk::Close(ChunkType::Square)
        } else if character == '{' {
            Chunk::Open(ChunkType::Curly)
        } else if character == '}' {
            Chunk::Close(ChunkType::Curly)
        } else if character == '<' {
            Chunk::Open(ChunkType::Angle)
        } else if character == '>' {
            Chunk::Close(ChunkType::Angle)
        } else if character == '(' {
            Chunk::Open(ChunkType::Round)
        } else if character == ')' {
            Chunk::Close(ChunkType::Round)
        } else {
            panic!("Invalid character: {}", character)
        }
    }
}

pub struct NavigationSubsystem {
    lines: Vec<String>
}

impl NavigationSubsystem {
    fn from_file(path: &str) -> NavigationSubsystem {
        let lines = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(Result::unwrap)
            .collect();
        NavigationSubsystem { lines }
    }

    fn calculate_score(&self) -> i64 {
        // let mut score = 0;
        let mut scores = Vec::new();
        for line in &self.lines {
            let score = NavigationSubsystem::line_score(&line);
            if score != 0 {
                scores.push(score);
            }
        }
        scores.sort();
        println!("All scores: {:?}", scores);
        scores[(scores.len()-1)/2]
    }

    fn line_score(line: &str) -> i64 {
        let mut stack = VecDeque::new();
        let mut score = 0i64;
        for character in line.chars() {
            match Chunk::from_char(character) {
                Chunk::Open(ctype) => stack.push_back(ctype),
                Chunk::Close(ctype) => {
                    let prev = stack.pop_back();
                    if let Some(ctype2) = prev {
                        if ctype != ctype2 {
                            if ctype == ChunkType::Round {
                                score = 3;
                            } else if ctype == ChunkType::Square {
                                score = 57;
                            } else if ctype == ChunkType::Curly {
                                score = 1197;
                            } else if ctype == ChunkType::Angle {
                                score = 25137;
                            }
                        }
                    }
                    if score != 0 {
                        // return score;
                    }
                }
            }
        }
        if score == 0 {
            println!("Calculating score for {:?}", stack);
            loop {
                if let Some(item) = stack.pop_back() {
                    score *= 5;
                    if item == ChunkType::Round {
                        score += 1;
                    } else if item == ChunkType::Square {
                        score += 2;
                    } else if item == ChunkType::Curly {
                        score += 3;
                    } else if item == ChunkType::Angle {
                        score += 4;
                    } else {
                        panic!("Unknown chunk type: {:?}", item);
                    }
                } else {
                    break;
                }
            }
            println!("Score is {}", score)
        } else {
            score = 0;
        }
        score
    }
}

fn main() {
    let subsystem = NavigationSubsystem::from_file("data/day10-input");
    // let subsystem = NavigationSubsystem::from_file("data/day10-sample");
    let score = subsystem.calculate_score();
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use crate::NavigationSubsystem;
    #[test]
    fn test_line_score() {
        assert_eq!(NavigationSubsystem::line_score("{{}}"), 0);
        assert_eq!(NavigationSubsystem::line_score("{{}]"), 57);
        assert_eq!(NavigationSubsystem::line_score("{{})"), 3);
    }
}