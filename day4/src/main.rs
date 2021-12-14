use std::fs::File;
use std::io::prelude::*;
use std::io;

#[derive(Copy, Clone, Debug)]
struct Board {
    data: [[(i32, bool); 5]; 5],
    win: bool,
}

impl Board {
    fn sum_unmarked(&self) -> i32 {
        self.data.iter().flat_map(|row| row.iter()).map(|x| if x.1 { 0 } else { x.0 }).sum()
    }

    fn apply_number(&mut self, num: &i32) -> bool {
        if self.win {
            // Boards can only win once
            return false
        }

        for y in 0..5 {
            let mut win_row = true;
            for x in 0..5 {
                if self.data[y][x].0 == *num {
                    self.data[y][x].1 = true;
                    
                    let mut win_col = true;
                    for y2 in 0..5 {
                        if !self.data[y2][x].1 {
                            win_col = false;
                            break;
                        }
                    }
                    if win_col {
                        self.win = true;
                        return true;
                    }
                }
                if !self.data[y][x].1 {
                    win_row = false;
                }
            }
            if win_row {
                self.win = true;
                return true;
            }
        }

        false
    }
}

fn part1(numbers: &Vec<i32>, boards_in: &Vec<Board>) -> io::Result<()> {
    let mut boards: Vec<Board> = boards_in.to_vec();
    for num in numbers {
        for board in boards.iter_mut() {
            if board.apply_number(num) {
                println!("Part 1: {}", board.sum_unmarked() * num);
                return Ok(())
            }
        }
    }
    panic!("Part 1: no boards win?");
}

fn part2(numbers: &Vec<i32>, boards_in: &Vec<Board>) -> io::Result<()> {
    let mut boards: Vec<Board> = boards_in.to_vec();
    let mut last_board: Option<(Board, i32)> = None;
    for num in numbers {
        for board in boards.iter_mut() {
            if board.apply_number(num) {
                last_board = Some((board.clone(), *num));
            }
        }
    }
    if let Some((board, num)) = last_board {
        println!("Part 2: {}", board.sum_unmarked() * num);
        return Ok(())
    } else {
        panic!("Part 2: no boards win?");
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let mut lines = contents.lines();
    let numbers = lines.next().unwrap().split(',').map(|line| line.parse::<i32>().unwrap() ).collect();
    let mut boards: Vec<Board> = vec!();

    while let Some(mut line) = lines.next() {
        if line.is_empty() {
            continue
        }

        let mut board = Board{data: [[(0i32, false); 5]; 5], win: false};
        for row in 0..5 {
            let nums: Vec<i32> = line.split_whitespace().map(|item| item.parse::<i32>().unwrap() ).collect();
            for col in 0..5 {
                board.data[row][col] = (nums[col], false);
            }
            if let Some(next_line) = lines.next() {
                line = next_line;
            } else if row != 4 {
                panic!("Not enough rows in board!");
            }
        }
        boards.push(board);
    }

    part1(&numbers, &boards)?;
    part2(&numbers, &boards)
}
