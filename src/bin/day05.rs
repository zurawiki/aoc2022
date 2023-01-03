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
    include_str!("../../data/day05/example.txt")
    // include_str!("../../data/day05/input.txt")
}

#[derive(Debug)]
struct Day05Data {
    stacks: Vec<Vec<u8>>,
    moves: Vec<(usize, usize, usize)>,
}

fn process_input() -> color_eyre::Result<Day05Data> {
    let (stack_state, moves_lines) = get_input().split_once("\n\n").unwrap();

    let mut stacks = stack_state
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|_| vec![])
        .collect::<Vec<Vec<u8>>>();
    stack_state.lines().rev().skip(1).for_each(|line| {
        let bytes = line.as_bytes();
        for i in 0..stacks.len() {
            let ch = bytes[1 + (4 * i)];
            if ch.is_ascii_alphabetic() {
                stacks.get_mut(i).unwrap().push(ch);
            }
        }
    });

    let moves = moves_lines
        .lines()
        .filter_map(|line| {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            // ex: move 5 from 8 to 3
            Some((
                words.get(1)?.parse().ok()?,
                words.get(3)?.parse().ok()?,
                words.get(5)?.parse().ok()?,
            ))
        })
        .collect::<Vec<(usize, usize, usize)>>();
    Ok(Day05Data { stacks, moves })
}

fn process_move(
    stacks: &mut Vec<Vec<u8>>,
    (n, from, to): (usize, usize, usize),
) -> color_eyre::Result<()> {
    for _ in 0..n {
        let item = stacks.get_mut(from - 1).unwrap().pop().unwrap();
        stacks.get_mut(to - 1).unwrap().push(item);
    }
    Ok(())
}

fn process_move_bulk(
    stacks: &mut Vec<Vec<u8>>,
    (n, from, to): (usize, usize, usize),
) -> color_eyre::Result<()> {
    let mut from_stack = stacks.get_mut(from - 1).unwrap();
    let mut items = from_stack.split_off(from_stack.len() - n);
    let mut to_stack = stacks.get_mut(to - 1).unwrap();
    to_stack.append(&mut items);
    Ok(())
}

fn part1() -> color_eyre::Result<()> {
    let mut data = process_input()?;
    for move_tuple in data.moves {
        process_move(&mut data.stacks, move_tuple)?;
    }
    println!(
        "Part 1 answer: {:?}",
        data.stacks
            .iter()
            .filter_map(|stack| stack.last().map(|ch| *ch as char))
            .collect::<String>()
    );
    Ok(())
}

fn part2() -> color_eyre::Result<()> {
    let mut data = process_input()?;
    for move_tuple in data.moves {
        process_move_bulk(&mut data.stacks, move_tuple)?;
    }
    println!(
        "Part 2 answer: {:?}",
        data.stacks
            .iter()
            .filter_map(|stack| stack.last().map(|ch| *ch as char))
            .collect::<String>()
    );
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    let part = Args::parse().part;
    println!("Parsed part = {}", part);

    match Args::parse().part {
        1 => part1()?,
        2 => part2()?,
        _ => panic!("part argument not recognized"),
    }

    Ok(())
}
