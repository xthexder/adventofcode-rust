use std::fs::File;
use std::io::prelude::*;
use std::io;

fn part1(numbers: &[i32]) -> io::Result<()> {
    let mut previous: Option<i32> = None;
    let mut sum = 0;
    for num in numbers {
        if let Some(prev) = previous {
            if *num > prev {
                sum += 1;
            }
        }
        previous = Some(*num);
    }
    println!("Part 1: {}", sum);
    Ok(())
}

fn part2(numbers: &[i32]) -> io::Result<()> {
    let sum = (3..numbers.len()).filter(|&head| numbers[head] > numbers[head - 3]).count();
    println!("Part 2: {}", sum);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let numbers: Vec<i32> = contents.lines().filter_map(|line| line.parse::<i32>().ok() ).collect();
    part1(&numbers)?;
    part2(&numbers)
}
