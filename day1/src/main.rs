use std::fs::File;
use std::io::prelude::*;
use std::io;

fn part1(contents: &str) -> io::Result<()> {
    let numbers = contents.lines().filter_map(|line| line.parse::<i32>().ok() );
    let mut previous: Option<i32> = None;
    let mut sum = 0;
    for num in numbers {
        if let Some(prev) = previous {
            if num > prev {
                sum += 1;
            }
        }
        previous = Some(num);
    }
    println!("Part 1: {}", sum);
    Ok(())
}

fn part2(contents: &str) -> io::Result<()> {
    let numbers: Vec<i32> = contents.lines().filter_map(|line| line.parse::<i32>().ok() ).collect();
    let (mut tail, mut head) = (0, 2);
    let mut sum = 0;
    while head < numbers.len() - 1 {
        head += 1;
        if numbers[head] - numbers[tail] > 0 {
            sum += 1;
        }
        tail += 1;
    }
    println!("Part 2: {}", sum);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    part1(&contents)?;
    part2(&contents)
}
