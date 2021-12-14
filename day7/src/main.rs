use std::fs::File;
use std::io::prelude::*;
use std::io;

fn part1(subs: &Vec<usize>) -> io::Result<()> {
    let mut min = *subs.iter().min().unwrap();
    let mut max = *subs.iter().max().unwrap();

    let mut sub_counts: Vec<usize> = vec![0; max+1];
    let mut fuel = 0usize;
    for sub in subs {
        sub_counts[*sub] += 1;
    }
    while min != max {
        if sub_counts[min] < sub_counts[max] {
            sub_counts[min+1] += sub_counts[min];
            fuel += sub_counts[min];
            sub_counts[min] = 0;
            min += 1;
        } else {
            sub_counts[max-1] += sub_counts[max];
            fuel += sub_counts[max];
            sub_counts[max] = 0;
            max -= 1;
        }
    }

    println!("Part 1: {}", fuel);
    return Ok(())
}

fn part2(subs: &Vec<usize>) -> io::Result<()> {
    let mut min = *subs.iter().min().unwrap();
    let mut max = *subs.iter().max().unwrap();

    let mut sub_counts: Vec<usize> = vec![0; max+1];
    let mut sub_costs: Vec<usize> = vec![0; max+1];
    let mut fuel = 0usize;
    for sub in subs {
        sub_counts[*sub] += 1;
        sub_costs[*sub] += 1;
    }
    while min != max {
        if sub_costs[min] < sub_costs[max] {
            sub_counts[min+1] += sub_counts[min];
            sub_costs[min+1] += sub_costs[min] + sub_counts[min];
            fuel += sub_costs[min];
            min += 1;
        } else {
            sub_counts[max-1] += sub_counts[max];
            sub_costs[max-1] += sub_costs[max] + sub_counts[max];
            fuel += sub_costs[max];
            max -= 1;
        }
    }

    println!("Part 2: {}", fuel);
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let subs = contents.split(',').map(|sub| sub.parse::<usize>().unwrap() ).collect::<Vec<_>>();

    part1(&subs)?;
    part2(&subs)
}
