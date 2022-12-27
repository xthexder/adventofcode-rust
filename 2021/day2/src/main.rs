use std::fs::File;
use std::io::prelude::*;
use std::io;

#[derive(Debug)]
struct Sub {
    position: i32,
    depth: i32,
    aim: i32,
}

fn part1(contents: &str) -> io::Result<()> {
    let commands = contents.lines().map(|line| {
        let mut split = line.split_whitespace();
        (split.next().unwrap(), split.next().unwrap().parse::<i32>().unwrap())
    });
    let mut sub = Sub{position: 0, depth: 0, aim: 0};
    for cmd in commands {
        match cmd.0 {
            "forward" => sub.position += cmd.1,
            "down" => sub.depth += cmd.1,
            "up" => sub.depth -= cmd.1,
            _ => panic!("Unexpected command: {}", cmd.0),
        }
    }
    println!("Part 1: {}", sub.position * sub.depth);
    Ok(())
}

fn part2(contents: &str) -> io::Result<()> {
    let commands = contents.lines().map(|line| {
        let mut split = line.split_whitespace();
        (split.next().unwrap(), split.next().unwrap().parse::<i32>().unwrap())
    });
    let mut sub = Sub{position: 0, depth: 0, aim: 0};
    for cmd in commands {
        match cmd.0 {
            "forward" => {
                sub.position += cmd.1;
                sub.depth += sub.aim * cmd.1
            },
            "down" => sub.aim += cmd.1,
            "up" => sub.aim -= cmd.1,
            _ => panic!("Unexpected command: {}", cmd.0),
        }
    }
    println!("Part 2: {}", sub.position * sub.depth);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    part1(&contents)?;
    part2(&contents)
}
