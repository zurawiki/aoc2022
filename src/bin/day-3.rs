use std::collections::HashSet;
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

pub fn score_letter(ch: u8) -> u32 {
    match ch {
        b'a'..=b'z' => (ch - b'a' + 1).into(),
        b'A'..=b'Z' => (ch - b'A' + 27).into(),
        _ => panic!("bad letter"),
    }
}

pub fn get_common_letter(a: &[u8], b: &[u8]) -> u8 {
    let a_set = a.iter().map(|&x| x).collect::<HashSet<u8>>();
    let b_set = b.iter().map(|&x| x).collect::<HashSet<u8>>();
    let mut intersection = a_set.intersection(&b_set);

    intersection.next().unwrap().clone()
}

pub fn get_common_letter3(a: &[u8], b: &[u8], c: &[u8]) -> u8 {
    let a_set = a.iter().map(|&x| x).collect::<HashSet<u8>>();
    let b_set = b.iter().map(|&x| x).collect::<HashSet<u8>>();
    let c_set = c.iter().map(|&x| x).collect::<HashSet<u8>>();
    let a_and_b_set = a_set
        .intersection(&b_set)
        .map(|&x| x)
        .collect::<HashSet<u8>>();
    let mut result = a_and_b_set.intersection(&c_set);

    result.next().unwrap().clone()
}

fn part1() {
    let mut result: u32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let bytes = line.unwrap().as_bytes().to_owned();
        let (first_half, second_half) = bytes.split_at(bytes.len() / 2);
        let common_letter = get_common_letter(&first_half, &second_half);

        result += score_letter(common_letter);
    }

    println!("result = {}", result);
}

fn part2() {
    let mut result: u32 = 0;

    let lines = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let chunks = lines.chunks(3);

    for chunk in chunks {
        let common_letter = get_common_letter3(
            chunk[0].as_bytes(),
            chunk[1].as_bytes(),
            chunk[2].as_bytes(),
        );

        result += score_letter(common_letter);
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_score_letter() {
        assert_eq!(score_letter(b'a'), 1);
        assert_eq!(score_letter(b'p'), 16);
        assert_eq!(score_letter(b'A'), 27);
        assert_eq!(score_letter(b'L'), 38);
        assert_eq!(score_letter(b'Z'), 52);
    }
}
