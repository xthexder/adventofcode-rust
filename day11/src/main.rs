use std::fs::File;
use std::io::prelude::*;
use std::io;

static DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

fn get_energy(grid: &Vec<Vec<u8>>, x: isize, y: isize) -> Option<u8> {
    if x < 0 || y < 0 {
        return None
    }
    Some(*grid.get(y as usize)?.get(x as usize)?)
}

fn _print_grid(grid: &Vec<Vec<u8>>) {
    for y in 0..grid.len() {
        grid[y].iter().for_each(|pos| {
            if *pos == 10 {
                print!("x");
            } else {
                print!("{}", pos);
            }
        });
        println!();
    }
    println!();
}

fn increase_energy(grid: &mut Vec<Vec<u8>>, x: isize, y: isize) {
    if let Some(start_energy) = get_energy(grid, x, y) {
        let pos = &mut grid[y as usize][x as usize];
        if start_energy <= 9 {
            *pos += 1;
            if *pos == 10 {
                for dir in DIRECTIONS {
                    increase_energy(grid, x + dir.0, y + dir.1);
                }
            }
        }
    }
}

fn part1(grid_in: &Vec<Vec<u8>>) -> io::Result<()> {
    let mut grid = grid_in.to_vec();

    let mut flashes = 0;
    for _ in 0..100 {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let (ix, iy) = (x as isize, y as isize);
                increase_energy(&mut grid, ix, iy);
            }
        }
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 10 {
                    flashes += 1;
                    grid[y][x] = 0;
                }
            }
        }
    }
    println!("Part 1: {}", flashes);
    return Ok(())
}
fn part2(grid_in: &Vec<Vec<u8>>) -> io::Result<()> {
    let mut grid = grid_in.to_vec();

    let mut step = 0;
    loop {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let (ix, iy) = (x as isize, y as isize);
                increase_energy(&mut grid, ix, iy);
            }
        }
        let mut all_flashes = true;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 10 {
                    grid[y][x] = 0;
                } else {
                    all_flashes = false;
                }
            }
        }
        step += 1;
        if all_flashes {
            break;
        }
    }
    println!("Part 2: {}", step);
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let grid = contents.lines().map(|line| {
        line.chars().map(|ch| {
            ch.to_string().parse::<u8>().unwrap()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    part1(&grid)?;
    part2(&grid)
}
