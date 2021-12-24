use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::fs::File;


enum Chunk {
    Open(ChunkType),
    Close(ChunkType),
}

#[derive(PartialEq, Eq)]
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

    fn calculate_score(&self) -> i32 {
        let mut score = 0;
        for line in &self.lines {
            score += NavigationSubsystem::line_score(&line);
        }
        score
    }

    fn line_score(line: &str) -> i32 {
        let mut stack = VecDeque::new();
        let mut score = 0;
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
                        return score;
                    }
                }
            }
        }
        0
    }
}

fn main() {
    let subsystem = NavigationSubsystem::from_file("data/day10-input");
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