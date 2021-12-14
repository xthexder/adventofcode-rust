use std::fs::File;
use std::io::prelude::*;
use std::io;

#[derive(Copy, Clone, PartialEq)]
struct Lanternfish {
    days_until_spawn: u8,
}

impl Lanternfish {
    fn advance_day(&mut self) -> Option<Lanternfish> {
        if self.days_until_spawn == 0 {
            // Spawn new
            self.days_until_spawn += 6;
            return Some(Lanternfish{ days_until_spawn: 8 });
        }
        self.days_until_spawn -= 1;
        None
    }
}

impl std::fmt::Debug for Lanternfish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.days_until_spawn)
    }
}

impl std::str::FromStr for Lanternfish {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lanternfish{ days_until_spawn: s.parse::<u8>()? })
    }
}

fn part1(fish_in: &Vec<Lanternfish>) -> io::Result<()> {
    let mut fish_list = fish_in.to_vec();
    let mut spawn_list: Vec<Lanternfish> = vec!();

    for _ in 0..80 {
        spawn_list.clear();
        for fish in fish_list.iter_mut() {
            if let Some(new_fish) = fish.advance_day() {
                spawn_list.push(new_fish);
            }
        }
        fish_list.extend(&spawn_list);
    }
    println!("Part 1: {}", fish_list.len());
    return Ok(())
}

fn part2(fish_list: &Vec<Lanternfish>) -> io::Result<()> {
    let mut spawn_count: [usize; 9] = [0; 9];
    for fish in fish_list {
        spawn_count[usize::from(fish.days_until_spawn)] += 1;
    }

    for _ in 0..256 {
        spawn_count = [
            spawn_count[1],
            spawn_count[2],
            spawn_count[3],
            spawn_count[4],
            spawn_count[5],
            spawn_count[6],
            spawn_count[7] + spawn_count[0],
            spawn_count[8],
            spawn_count[0],
        ];
    }
    println!("Part 2: {}", spawn_count.iter().sum::<usize>());
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let fish = contents.split(',').map(|fish| fish.parse::<Lanternfish>().unwrap() ).collect::<Vec<_>>();

    part1(&fish)?;
    part2(&fish)
}
