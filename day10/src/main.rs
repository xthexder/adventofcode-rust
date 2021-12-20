use std::fs::File;
use std::io::prelude::*;
use std::io;

fn part1(lines: &Vec<&str>) -> io::Result<()> {
    let mut score = 0;
    for line in lines {
        let mut stack: Vec<char> = vec!();
        for ch in line.chars() {
            let stack_top = stack.last();
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => {
                    if Some(&ch) == stack_top {
                        stack.pop();
                    } else {
                        score += match ch {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Unexpected character: {}", ch),
                        };
                        break;
                    }
                },
            }
        }
    }
    println!("Part 1: {}", score);
    return Ok(())
}

fn part2(lines: &Vec<&str>) -> io::Result<()> {
    let mut scores: Vec<isize> = vec!();
    for line in lines {
        let mut stack: Vec<char> = vec!();
        for ch in line.chars() {
            let stack_top = stack.last();
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => {
                    if Some(&ch) == stack_top {
                        stack.pop();
                    } else {
                        stack.clear();
                        break;
                    }
                },
            }
        }
        if stack.len() > 0 {
            let mut score = 0isize;
            for ch in stack.iter().rev() {
                score = score * 5 + match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Unexpected character: {}", ch),
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("Part 2: {:?}", scores[scores.len()/2]);
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let lines = contents.lines().collect::<Vec<_>>();

    part1(&lines)?;
    part2(&lines)
}
