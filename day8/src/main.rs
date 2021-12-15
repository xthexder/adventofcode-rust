use std::fs::File;
use std::io::prelude::*;
use std::io;

#[derive(Copy, Clone, PartialEq)]
struct Digit {
    segments: u8,
    number: u8,
}

impl Digit {
    fn get_number(&self, one: u8, four: u8) -> u8 {
        if self.segments.count_ones() == 5 {
            if self.segments | one == self.segments {
                return 3;
            } else if self.segments | four == 0b1111111 {
                return 2;
            } else {
                return 5;
            }
        } else if self.segments.count_ones() == 6 {
            if self.segments | one != self.segments {
                return 6;
            } else if self.segments | four == self.segments {
                return 9;
            } else {
                return 0;
            }
        }
        if self.number == 0xFF {
            panic!("Unknown number: {:07b}", self.segments);
        }
        return self.number;
    }
}

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.number != 0xFF {
            write!(f, "{:7}", self.number)
        } else {
            write!(f, "{:07b}", self.segments)
        }
    }
}

impl std::str::FromStr for Digit {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = 0u8;
        s.bytes().for_each(|ch| segments |= 1 << (ch - 97u8 /* 'a' */) );
        Ok(Digit{ segments: segments, number: match segments.count_ones() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => 0xFF,
        }})
    }
}

#[derive(Clone, PartialEq)]
struct Instance {
    input: Vec<Digit>,
    output: Vec<Digit>,
}

impl Instance {
    fn get_number(&self, one: u8, four: u8) -> u32 {
        let mut result = 0u32;
        for digit in &self.output {
            result = result * 10 + digit.get_number(one, four) as u32;
        }
        result
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} | {:?}", self.input, self.output)
    }
}

impl std::str::FromStr for Instance {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" | ").map(|part| part.split_whitespace().map( |digit| digit.parse::<Digit>().unwrap() ));
        let mut input = parts.next().unwrap().collect::<Vec<_>>();
        let output = parts.next().unwrap().collect::<Vec<_>>();
        input.extend(output.iter());
        Ok(Instance{ input: input, output: output })
    }
}

fn part1(lines: &Vec<Instance>) -> io::Result<()> {
    let digits = lines.iter().flat_map(|line| line.output.to_vec() );
    let sum = digits.fold(0, |sum, digit| if digit.number == 0xFF { sum } else { sum + 1 } );
    println!("Part 1: {}", sum);
    return Ok(())
}

fn part2(lines: &Vec<Instance>) -> io::Result<()> {
    let mut sum = 0u32;
    for line in lines {
        let (mut one, mut four) = (!0u8, !0u8);
        line.input.iter().for_each(|digit| match digit.number {
            1 => one = digit.segments,
            4 => four = digit.segments,
            _ => (),
        });
        sum += line.get_number(one, four);
    }
    println!("Part 2: {}", sum);
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let lines = contents.lines().map(|line| line.parse::<Instance>().unwrap() ).collect::<Vec<_>>();

    part1(&lines)?;
    part2(&lines)
}
