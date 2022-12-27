use std::fs::File;
use std::io::prelude::*;
use std::io;

fn do_fold(dots: &mut Vec<(i32, i32)>, fold: &(bool, i32)) {
    for (x, y) in dots.iter_mut() {
        if fold.0 {
            if *x > fold.1 {
                *x = fold.1 - *x + fold.1;
            }
        } else {
            if *y > fold.1 {
                *y = fold.1 - *y + fold.1;
            }
        }
    }
}

fn part1(dots_in: &[(i32, i32)], folds: &[(bool, i32)]) -> io::Result<()> {
    let mut dots = dots_in.to_vec();
    do_fold(&mut dots, &folds[0]);
    dots.sort();
    dots.dedup();
    println!("Part 1: {}", dots.len());
    return Ok(())
}
fn part2(dots_in: &[(i32, i32)], folds: &[(bool, i32)]) -> io::Result<()> {
    let mut dots = dots_in.to_vec();
    for fold in folds {
        do_fold(&mut dots, fold);
    }
    let mut min_opt = (None::<i32>, None::<i32>);
    let mut max_opt = (None::<i32>, None::<i32>);
    for (x, y) in &dots {
        min_opt.0 = Some(std::cmp::min(min_opt.0.unwrap_or(*x), *x));
        min_opt.1 = Some(std::cmp::min(min_opt.1.unwrap_or(*y), *y));
        max_opt.0 = Some(std::cmp::max(max_opt.0.unwrap_or(*x), *x));
        max_opt.1 = Some(std::cmp::max(max_opt.1.unwrap_or(*y), *y));
    }
    let min = (min_opt.0.unwrap(), min_opt.1.unwrap());
    let max = (max_opt.0.unwrap(), max_opt.1.unwrap());
    let mut grid = vec![vec![false; (max.0 - min.0 + 1) as usize]; (max.1 - min.1 + 1) as usize];
    for (x, y) in &dots {
        grid[*y as usize][*x as usize] = true;
    }
    println!("Part 2:");
    for row in &grid {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let dots = contents.lines().filter_map(|line| {
        line.split_once(',').map(|(x, y)| {
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
    }).collect::<Vec<_>>();
    let folds = contents.lines().filter_map(|line| {
        line.split_once('=').map(|(axis, num)| {
            (axis.ends_with("x"), num.parse::<i32>().unwrap())
        })
    }).collect::<Vec<_>>();

    part1(&dots, &folds)?;
    part2(&dots, &folds)
}
