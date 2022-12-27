use std::fs::File;
use std::io::prelude::*;
use std::io;

static DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_height(grid: &Vec<Vec<u8>>, x: isize, y: isize) -> Option<u8> {
    if x < 0 || y < 0 {
        return None
    }
    Some(*grid.get(y as usize)?.get(x as usize)?)
}

fn is_low_point(grid: &Vec<Vec<u8>>, x: isize, y: isize) -> bool {
    if let Some(center_height) = get_height(grid, x, y) {
        let (min_neighbor, max_neighbor) = DIRECTIONS.iter().fold((center_height, center_height), |(mut min, mut max), dir| {
            if let Some(height) = get_height(grid, x + dir.0, y + dir.1) {
                if height > max {
                    max = height;
                }
                if height < min {
                    min = height;
                }
            }
            (min, max)
        });
        if min_neighbor < center_height {
            return false;
        }
        return max_neighbor > center_height;
    }
    false
}

fn part1(grid: &Vec<Vec<u8>>) -> io::Result<Vec<(isize, isize)>> {
    let mut sum = 0;
    let mut low_points: Vec<(isize, isize)> = vec!();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let (ix, iy) = (x as isize, y as isize);
            if is_low_point(grid, ix, iy) {
                sum += 1 + get_height(grid, ix, iy).unwrap_or(0) as u32;
                low_points.push((ix, iy));
            }
        }
    }
    println!("Part 1: {}", sum);
    return Ok(low_points)
}

#[derive(Debug)]
struct Basin {
    x: isize,
    y: isize,
    size: usize,
}

impl Basin {
    fn fill(&mut self, grid: &mut Vec<Vec<u8>>) {
        self.fill_internal(grid, self.x, self.y)
    }

    fn fill_internal(&mut self, grid: &mut Vec<Vec<u8>>, x: isize, y: isize) {
        if get_height(&grid, x, y) == Some(0xFF) {
            self.size += 1;
            grid[y as usize][x as usize] = 0xFE;
            for dir in DIRECTIONS {
                self.fill_internal(grid, x + dir.0, y + dir.1);
            }
        }
    }
}

fn part2(grid_in: &Vec<Vec<u8>>, low_points: &Vec<(isize, isize)>) -> io::Result<()> {
    let mut grid = grid_in.to_vec();
    for point in low_points {
        grid[point.1 as usize][point.0 as usize] = 0xFF;
    }
    for layer in 0..9 {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == layer {
                    let (ix, iy) = (x as isize, y as isize);
                    let near_basin = DIRECTIONS.iter().find(|dir| {
                        if get_height(&grid, ix + dir.0, iy + dir.1) == Some(0xFF) {
                            return true;
                        }
                        false
                    });
                    if near_basin.is_some() {
                        grid[y][x] = 0xFF;
                    }
                }
            }
        }
    }
    let mut basins = low_points.iter().map(|point| Basin{x: point.0, y: point.1, size: 0}).collect::<Vec<_>>();
    basins.iter_mut().for_each(|basin| basin.fill(&mut grid) );
    basins.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    let multiplied_sizes = basins[0..3].iter().fold(None, |opt, basin| {
        if let Some(product) = opt {
            Some(product * basin.size)
        } else {
            Some(basin.size)
        }
    }).unwrap();
    println!("Part 2: {:?}", multiplied_sizes);
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

    let low_points = part1(&grid)?;
    part2(&grid, &low_points)
}
