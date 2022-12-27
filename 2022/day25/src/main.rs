use std::fs::File;
use std::io;
use std::io::prelude::*;

fn snafu_to_int(snafu: &Vec<i8>) -> i64 {
    let mut total = 0;
    let mut multiplier = 1;
    for ch in snafu.iter().rev() {
        total += *ch as i64 * multiplier;
        multiplier *= 5;
    }
    return total;
}

fn int_to_snafu(int: i64) -> Vec<i8> {
    let mut snafu = vec![0];
    let mut remainder = int;
    while remainder < -2 || remainder > 2 {
        let mut limit = 12;
        let mut multiplier = 5;
        let mut index = 1;
        while remainder < -limit || remainder > limit {
            multiplier *= 5;
            limit += multiplier * 2;
            index += 1;
        }
        if index + 1 >= snafu.len() {
            snafu.resize(index + 1, 0);
        }
        // println!(
        //     "{} index {} multiplier {} limit {}",
        //     remainder, index, multiplier, limit
        // );

        index = snafu.len() - index - 1;
        while remainder >= multiplier {
            snafu[index] += 1;
            remainder -= multiplier;
        }
        if snafu[index] < 2 && remainder - multiplier > -remainder {
            snafu[index] += 1;
            remainder -= multiplier;
        }
        while remainder <= -multiplier {
            snafu[index] -= 1;
            remainder += multiplier;
        }
        if snafu[index] > -2 && remainder + multiplier < -remainder {
            snafu[index] -= 1;
            remainder += multiplier;
        }
        // println!("remainder {} index {} = {}", remainder, index, snafu[index]);
    }
    let index = snafu.len() - 1;
    snafu[index] = remainder as i8;
    return snafu;
}

fn snafu_to_str(snafu: &Vec<i8>) -> String {
    snafu
        .iter()
        .map(|&x| match x {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => 'x',
        })
        .collect()
}

fn part1(input: &[Vec<i8>]) -> io::Result<()> {
    let mut sum = 0;
    for snafu in input {
        // println!("{}: {}", snafu_to_str(snafu), snafu_to_int(snafu));
        sum += snafu_to_int(snafu);
    }
    // for i in 0..101 {
    //     println!("{}: {}", i, snafu_to_str(&int_to_snafu(i)));
    // }
    // println!("{}: {}", 2022, snafu_to_str(&int_to_snafu(2022)));
    // println!("{}: {}", 12345, snafu_to_str(&int_to_snafu(12345)));
    // println!("{}: {}", 314159265, snafu_to_str(&int_to_snafu(314159265)));
    println!("Part 1: {} = {}", sum, snafu_to_str(&int_to_snafu(sum)));
    return Ok(());
}
// fn part2(dots_in: &[(i32, i32)], folds: &[(bool, i32)]) -> io::Result<()> {
//     let mut dots = dots_in.to_vec();
//     for fold in folds {
//         do_fold(&mut dots, fold);
//     }
//     let mut min_opt = (None::<i32>, None::<i32>);
//     let mut max_opt = (None::<i32>, None::<i32>);
//     for (x, y) in &dots {
//         min_opt.0 = Some(std::cmp::min(min_opt.0.unwrap_or(*x), *x));
//         min_opt.1 = Some(std::cmp::min(min_opt.1.unwrap_or(*y), *y));
//         max_opt.0 = Some(std::cmp::max(max_opt.0.unwrap_or(*x), *x));
//         max_opt.1 = Some(std::cmp::max(max_opt.1.unwrap_or(*y), *y));
//     }
//     let min = (min_opt.0.unwrap(), min_opt.1.unwrap());
//     let max = (max_opt.0.unwrap(), max_opt.1.unwrap());
//     let mut grid = vec![vec![false; (max.0 - min.0 + 1) as usize]; (max.1 - min.1 + 1) as usize];
//     for (x, y) in &dots {
//         grid[*y as usize][*x as usize] = true;
//     }
//     println!("Part 2:");
//     for row in &grid {
//         for cell in row {
//             if *cell {
//                 print!("#");
//             } else {
//                 print!(" ");
//             }
//         }
//         println!();
//     }
//     return Ok(())
// }

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let input = contents
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.chars()
                        .filter_map(|ch| match ch {
                            '2' => Some(2),
                            '1' => Some(1),
                            '0' => Some(0),
                            '-' => Some(-1),
                            '=' => Some(-2),
                            _ => None,
                        })
                        .collect::<Vec<i8>>(),
                )
            }
        })
        .collect::<Vec<_>>();

    part1(&input)
    // part2(&input)
}
