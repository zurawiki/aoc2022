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

#[derive(Debug, Clone, Copy)]
enum RPSMove {
    Paper,
    Rock,
    Scissors,
}
use crate::RPSMove::*;

fn calculate_score(them: RPSMove, you: RPSMove) -> u32 {
    let outcome_score = match (them, you) {
        (Paper, Paper) => 3,
        (Paper, Rock) => 0,
        (Paper, Scissors) => 6,
        (Rock, Paper) => 6,
        (Rock, Rock) => 3,
        (Rock, Scissors) => 0,
        (Scissors, Paper) => 0,
        (Scissors, Rock) => 6,
        (Scissors, Scissors) => 3,
    };
    let select_score = match you {
        Paper => 2,
        Rock => 1,
        Scissors => 3,
    };
    outcome_score + select_score
}

fn part1() {
    let mut result: u32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let bytes = line.unwrap().as_bytes().to_owned();
        let (them_b, you_b) = (bytes[0], bytes[2]);
        let them = match them_b {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            _ => panic!("input {:?} not recognized", bytes),
        };
        let you = match you_b {
            b'X' => Rock,
            b'Y' => Paper,
            b'Z' => Scissors,
            _ => panic!("input {:?} not recognized", bytes),
        };
        println!(
            "calculate_score({:?},{:?}) = {}",
            them,
            you,
            calculate_score(them, you)
        );

        result += calculate_score(them, you);
    }

    println!("result = {}", result);
}

fn part2() {
    let mut result: u32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let bytes = line.unwrap().as_bytes().to_owned();
        let (them_b, you_b) = (bytes[0], bytes[2]);
        let them = match them_b {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            _ => panic!("input {:?} not recognized", bytes),
        };
        let you = match (them, you_b) {
            (_, b'Y') => them,
            (Rock, b'X') => Scissors,
            (Paper, b'X') => Rock,
            (Scissors, b'X') => Paper,
            (Rock, b'Z') => Paper,
            (Paper, b'Z') => Scissors,
            (Scissors, b'Z') => Rock,

            _ => panic!("input {:?} not recognized", bytes),
        };
        println!(
            "calculate_score({:?},{:?}) = {}",
            them,
            you,
            calculate_score(them, you)
        );

        result += calculate_score(them, you);
    }

    println!("result = {}", result);
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
