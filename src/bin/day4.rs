use std::fs::File;
use std::io::{BufRead, BufReader};
use ndarray::Array2;

struct Bingo  {
    numbers: Vec<i32>,
    boards: Vec<Array2<i32>>
}

struct BingoResults<'t> {
    bingo: &'t Bingo,
    results: Vec<Array2<i32>>,
}

impl<'t> BingoResults<'t> {
    fn new(bingo: &Bingo) -> BingoResults {
        let mut results = Vec::new();
        for ind in 0..bingo.boards.len() {
            results.push(Array2::<i32>::zeros((5, 5)));
        }
        let output = BingoResults { bingo, results };
        output
    }

    fn play_number(&mut self, number: &i32) {
        println!("Playing number: {}", number);
        for (boardind, board) in self.bingo.boards.iter().enumerate() {
            for (rowind, row) in board.rows().into_iter().enumerate() {
                for (colind, val) in row.iter().enumerate() {
                    if val == number {
                        self.results[boardind][[rowind, colind]] = 1;
                    }
                }
            }
        }
    }

    fn sum_unmarked(&self, boardind: usize) -> i32 {
        let mut sum = 0;
        for (number, marked) in self.bingo.boards[boardind].iter().zip(self.results[boardind].iter()) {
            if *marked == 0 {
                sum += *number;
            }
        }
        sum
    }

    fn check_boards(&self) -> i32 {
        for (boardind, board) in self.results.iter().enumerate() {
            for row in board.rows() {
                if row.iter().all(|val| *val == 1) {
                    println!("Bingo! on board {}", boardind);
                    return self.sum_unmarked(boardind);
                }
            }
            for col in board.columns() {
                if col.iter().all(|val| *val == 1) {
                    println!("Bingo! on board {}", boardind);
                    return self.sum_unmarked(boardind);
                }
            }
        }
        -1
    }
}

fn read_bingo(path: &str) -> Bingo {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines();
    let firstline = lines.next().unwrap().unwrap();
    let numbers: Vec<i32> = firstline.split(",").map(|num| num.parse::<i32>().unwrap()).collect();
    let mut rowind = 0;
    let mut boards: Vec<Array2<i32>> = Vec::new();
    let mut currboard = Array2::<i32>::zeros((5, 5));
    for line in lines {
        let line = line.unwrap();
        if line.len() > 0 {
            for (colind, val) in line.split_whitespace().enumerate() {
                currboard[[rowind, colind]] = val.parse::<i32>().unwrap();
            }
            rowind += 1;
            if rowind == 5 {
                boards.push(currboard);
                currboard = Array2::<i32>::zeros((5, 5));
                rowind = 0;
            }
        }
    }
    println!("Found {} boards and numbers: {:?}", boards.len(), numbers);
    println!("Final board: {:?}", boards[boards.len()-1]);
    Bingo { numbers, boards }
}

fn play_boards(bingo: &Bingo) -> i32 {
    let mut results = BingoResults::new(&bingo);
    for number in &bingo.numbers {
        println!("playing {}", number);
        results.play_number(number);
        println!("Board 1 results: {:?}", results.results[0]);
        let sum_unmarked = results.check_boards();
        if sum_unmarked != -1 {
            println!("Sum of unmarked: {}, last number: {}", sum_unmarked, number);
            return sum_unmarked * number;
        }
    }
    0
}


fn main() {
    let bingo = read_bingo("data/day4-input");
    let final_score = play_boards(&bingo);
    println!("Final score! {}", final_score);
}