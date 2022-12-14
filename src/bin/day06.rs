use std::collections::HashSet;

use clap::Parser;

/// Argument template for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify part to compute (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn get_input() -> &'static str {
    // include_str!("../../data/day06/example.txt")
    include_str!("../../data/day06/input.txt")
}

fn part1() -> color_eyre::Result<()> {
    let n_distinct = 4usize;

    let mut i = n_distinct;
    for window in get_input().as_bytes().windows(n_distinct) {
        let set: HashSet<u8> = HashSet::from_iter(window.iter().map(|b| *b));
        if set.len() == n_distinct {
            println!("Part 1 answer: {:?}", i);
            break;
        }
        i += 1;
    }
    println!("Part 1 done");
    Ok(())
}

fn part2() -> color_eyre::Result<()> {
    let n_distinct = 14usize;
    let mut i = n_distinct;
    for window in get_input().as_bytes().windows(n_distinct) {
        let set: HashSet<u8> = HashSet::from_iter(window.iter().map(|b| *b));
        if set.len() == n_distinct {
            println!("Part 2 answer: {:?}", i);
            break;
        }
        i += 1;
    }
    println!("Part 2 done");
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    let part = Args::parse().part;
    println!("Running for part{}", part);

    match Args::parse().part {
        1 => part1()?,
        2 => part2()?,
        _ => panic!("part argument not recognized"),
    }

    Ok(())
}
