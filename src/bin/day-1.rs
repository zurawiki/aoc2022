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
fn part2() {}

fn main() {
    let part = Args::parse().part;
    println!("Parsed part = {}", part);

    match Args::parse().part {
        1 => part1(),
        2 => part2(),
        _ => panic!("part argument not recognized"),
    }
}
