use std::fs::File;
use std::io::prelude::*;
use std::io;

fn part1(contents: &str) -> io::Result<Vec<(i32, i32)>> {
    let line_len = contents.lines().nth(0).unwrap().len();
    let mut counts = vec![(0, 0); line_len];

    for line in contents.lines() {
        for (i, c) in line.char_indices() {
            if c == '1' {
                counts[i].1 += 1
            } else {
                counts[i].0 += 1
            }
        }
    }

    let (gamma, epsilon) = counts.iter().fold((0, 0), |acc, bits| {
        if bits.1 > bits.0 {
            ((acc.0 << 1) + 1, acc.1 << 1)
        } else {
            (acc.0 << 1, (acc.1 << 1) + 1)
        }
    });
    println!("Part 1: {0}", gamma * epsilon);
    Ok(counts)
}

fn part2(contents: &str) -> io::Result<()> {
    let line_len = contents.lines().nth(0).unwrap().len();
    let lines = contents.lines().collect::<Vec<_>>();
    let (mut oxygen, mut co2) = (lines.clone(), lines);

    for i in 0..line_len {
        if oxygen.len() > 1 {
            let mut o2_count = (0, 0);
            for line in oxygen.iter() {
                if line.chars().nth(i) == Some('1') {
                    o2_count.1 += 1
                } else {
                    o2_count.0 += 1
                }
            }

            oxygen = oxygen.into_iter().filter(|line| {
                if o2_count.1 >= o2_count.0 {
                    line.chars().nth(i) == Some('1')
                } else {
                    line.chars().nth(i) == Some('0')
                }
            }).collect();
        }
        
        if co2.len() > 1 {
            let mut co2_count = (0, 0);
            for line in co2.iter() {
                if line.chars().nth(i) == Some('1') {
                    co2_count.1 += 1
                } else {
                    co2_count.0 += 1
                }
            }

            co2 = co2.into_iter().filter(|line| {
                if co2_count.1 < co2_count.0 {
                    line.chars().nth(i) == Some('1')
                } else {
                    line.chars().nth(i) == Some('0')
                }
            }).collect();
        }

        if oxygen.len() <= 1 && co2.len() <= 1 {
            break
        }
    }
    let result = i32::from_str_radix(oxygen[0], 2).unwrap() * i32::from_str_radix(co2[0], 2).unwrap();
    println!("Part 2: {}", result);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    part1(&contents)?;
    part2(&contents)
}
