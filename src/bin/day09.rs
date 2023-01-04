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
    // include_str!("../../data/day09/example.txt")
    include_str!("../../data/day09/input.txt")
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn update_location((x, y): (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn is_touching((head_x, head_y): (i32, i32), (tail_x, tail_y): (i32, i32)) -> bool {
    let x_diff = (head_x - tail_x).abs();
    let y_diff = (head_y - tail_y).abs();
    std::cmp::max(x_diff, y_diff) <= 1
}

fn part1() -> color_eyre::Result<()> {
    let instructions: Vec<(Direction, i32)> = get_input()
        .lines()
        .map(|line| {
            let (dir, length) = line.split_once(' ').unwrap();
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("unknown direction"),
            };
            let length = length.parse::<i32>().unwrap();
            dbg!((dir, length))
        })
        .collect::<Vec<_>>();

    let mut tail_spaces: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    tail_spaces.insert(tail);

    for (dir, length) in instructions {
        println!("dir: {:?}, length: {:?}", dir, length);
        for _ in 0..length {
            let new_head = update_location(head, dir);
            if !is_touching(new_head, tail) {
                tail = head;
                tail_spaces.insert(tail);
            }
            head = new_head;

        
        }
    }
    dbg!(tail_spaces.len());
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
