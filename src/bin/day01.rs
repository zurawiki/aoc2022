use std::cmp::max;
use std::io;
use std::io::BufRead;

use clap::Parser;

/// Argument template for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify part to compute (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn part1() {
    let mut max_sum: u64 = 0;
    let mut curr_sum: u64 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(value) = line.unwrap().parse::<u64>() {
            curr_sum += value;
        } else {
            max_sum = max(max_sum, curr_sum);
            curr_sum = 0;
        }
    }

    max_sum = max(max_sum, curr_sum);
    println!("{}", max_sum);
}

fn max3<T>(vec: &mut Vec<T>, val: T)
where
    T: Ord,
{
    vec.push(val);
    vec.sort_by(|a, b| b.cmp(a));
    vec.truncate(3);
}

fn part2() {
    let mut max3_sum: Vec<u64> = vec![0, 0, 0];
    let mut curr_sum: u64 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(value) = line.unwrap().parse::<u64>() {
            curr_sum += value;
        } else {
            max3(&mut max3_sum, curr_sum);
            curr_sum = 0;
        }
    }

    max3(&mut max3_sum, curr_sum);
    println!(
        "Array: {:#?}, Sum = {}",
        max3_sum,
        max3_sum.iter().sum::<u64>()
    );
}

fn main() {
    let part = Args::parse().part;
    println!("Parsed part = {}", part);

    match Args::parse().part {
        1 => part1(),
        2 => part2(),
        _ => panic!("part argument not recognized"),
    }
}
