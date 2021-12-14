use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',').map( |num| num.parse::<usize>() );

        Ok(Point{x: coords.next().unwrap()?, y: coords.next().unwrap()?})
    }
}

#[derive(Copy, Clone, Debug)]
struct Line (Point, Point);

impl std::str::FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ").map( |point| point.parse::<Point>() );

        Ok(Line(points.next().unwrap()?, points.next().unwrap()?))
    }
}

#[derive(Debug)]
struct Board {
    data: Vec<Vec<u32>>,
    min: Point,
    max: Point,
}

impl Board {
    fn mark_line(&mut self, line: &Line, horizontal_only: bool) {
        if horizontal_only {
            if line.0.x != line.1.x && line.0.y != line.1.y {
                return
            }
        }

        self.min.x = cmp::min(cmp::min(self.min.x, line.0.x), line.1.x);
        self.min.y = cmp::min(cmp::min(self.min.y, line.0.y), line.1.y);
        self.max.x = cmp::max(cmp::max(self.max.x, line.0.x), line.1.x);
        self.max.y = cmp::max(cmp::max(self.max.y, line.0.y), line.1.y);

        // println!("Marking line: {:?} to {:?}", line.0, line.1);
        let mut p: Point = line.0;
        while p != line.1 {
            self.data[p.y][p.x] += 1;
            if p.x < line.1.x {
                p.x += 1
            } else if p.x > line.1.x {
                p.x -= 1
            }
            if p.y < line.1.y {
                p.y += 1
            } else if p.y > line.1.y {
                p.y -= 1
            }
        }
        self.data[p.y][p.x] += 1
    }

    fn num_overlaps(&self) -> usize {
        let mut count: usize = 0;
        for row in self.data.iter() {
            for cell in row {
                if *cell > 1 {
                    count += 1
                }
            }
        }
        count
    }

    fn _print_board(&self) {
        for y in self.min.y..=self.max.y {
            println!("{:?}", &self.data[y][self.min.x..=self.max.x]);
        }
    }
}

fn part1(lines: &Vec<Line>) -> io::Result<()> {
    let mut board = Board{
        data: vec![vec![0; 1000]; 1000],
        min: Point{x: 0, y: 0},
        max: Point{x: 0, y: 0},
    };

    for line in lines {
        board.mark_line(line, true);
    }
    // board._print_board();
    
    println!("Part 1: {}", board.num_overlaps());
    return Ok(())
}

fn part2(lines: &Vec<Line>) -> io::Result<()> {
    let mut board = Board{
        data: vec![vec![0; 1000]; 1000],
        min: Point{x: 0, y: 0},
        max: Point{x: 0, y: 0},
    };

    for line in lines {
        board.mark_line(line, false);
    }
    // board._print_board();
    
    println!("Part 2: {}", board.num_overlaps());
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let lines = contents.lines().map(|line| line.parse::<Line>().unwrap() ).collect::<Vec<_>>();

    part1(&lines)?;
    part2(&lines)
}
