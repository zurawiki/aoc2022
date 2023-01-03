use clap::Parser;
mod day07_parser;

/// Argument template for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify part to compute (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn get_input() -> &'static str {
    // include_str!("../../data/day07/example.txt")
    include_str!("../../data/day07/input.txt")
}

fn parse_input() -> color_eyre::Result<Vec<day07_parser::FsNode>> {
    let input = get_input();
    Ok(day07_parser::cd_output(input).map(|(_, r)| r)?)
}

use day07_parser::FsNode;

const MAX_SIZE: u64 = 100000;

fn part1() -> color_eyre::Result<()> {
    let mut root = dbg!(parse_input()).unwrap();
    let node = root.pop().unwrap();

    let dirs: Vec<_> = node
        .into_iter()
        .filter_map(|n| {
            if let FsNode::File { .. } = n {
                return None;
            }
            let size = n.total_size();
            if size <= MAX_SIZE {
                Some(size)
            } else {
                None
            }
        })
        .collect();

    let answer = dirs.iter().sum::<u64>();

    println!("Part 1 dirs= {:?}", dirs);
    println!("Part 1 answer= {:?}", answer);
    println!("Part 1 done");
    Ok(())
}

fn part2() -> color_eyre::Result<()> {
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let part = Args::parse().part;
    println!("Running for part{}", part);
    match Args::parse().part {
        1 => part1()?,
        2 => part2()?,
        _ => panic!("part argument not recognized"),
    }
    Ok(())
}
