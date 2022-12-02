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
   
}

fn part2() {
    
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
