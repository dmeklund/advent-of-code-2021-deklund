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
    completed: Vec<bool>,
}

impl<'t> BingoResults<'t> {
    fn new(bingo: &Bingo) -> BingoResults {
        let mut results = Vec::new();
        for ind in 0..bingo.boards.len() {
            results.push(Array2::<i32>::zeros((5, 5)));
        }
        let completed = vec![false; bingo.boards.len()];
        let output = BingoResults { bingo, results, completed };
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

    fn check_boards(&mut self, find_last: bool) -> i32 {
        let mut is_final_board = false;
        let mut last_board_ind = 0;
        if find_last {
            let mut count = 0;
            for (boardind, is_complete) in self.completed.iter().enumerate() {
                if !is_complete {
                    count += 1;
                    last_board_ind = boardind;
                }
                if count > 1 {
                    break;
                }
            }
            if count == 1 {
                println!("We're on the final board ({}) - if it bingos we'll return", last_board_ind);
                is_final_board = true;
            }
        }
        for (boardind, board) in self.results.iter().enumerate() {
            for row in board.rows() {
                if row.iter().all(|val| *val == 1) {
                    println!("Bingo! on board {}", boardind);
                    self.completed[boardind] = true;
                    if !find_last || (is_final_board && boardind == last_board_ind) {
                        return self.sum_unmarked(boardind);
                    }
                }
            }
            for col in board.columns() {
                if col.iter().all(|val| *val == 1) {
                    self.completed[boardind] = true;
                    println!("Bingo! on board {}", boardind);
                    if !find_last || (is_final_board && boardind == last_board_ind)  {
                        return self.sum_unmarked(boardind);
                    }
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

fn play_boards(bingo: &Bingo, find_last: bool) -> i32 {
    let mut results = BingoResults::new(&bingo);
    for number in &bingo.numbers {
        println!("playing {}", number);
        results.play_number(number);
        println!("Board 1 results: {:?}", results.results[0]);
        let sum_unmarked = results.check_boards(find_last);
        if sum_unmarked != -1 {
            println!("Sum of unmarked: {}, last number: {}", sum_unmarked, number);
            return sum_unmarked * number;
        }
    }
    0
}


fn main() {
    let bingo = read_bingo("data/day4-input");
    let find_last = true;
    let final_score = play_boards(&bingo, find_last);
    println!("Final score! {}", final_score);
}

