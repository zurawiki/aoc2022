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

fn part1() -> color_eyre::Result<()> {
    let full_overlaps = include_str!("../../data/day04/input.txt")
        .lines()
        .filter_map(|line| {
            let ranges = line.split_once(',')?;
            let first = ranges.0.split_once('-')?;
            let (x1, y1) = (
                first.0.parse::<u32>().unwrap(),
                first.1.parse::<u32>().unwrap(),
            );

            let second = ranges.1.split_once('-')?;
            let (x2, y2) = (
                second.0.parse::<u32>().unwrap(),
                second.1.parse::<u32>().unwrap(),
            );
            let is_contained = (x1 <= x2 && y1 >= y2) || (x2 <= x1 && y2 >= y1);
            dbg!((line, is_contained));

            if is_contained {
                Some(())
            } else {
                None
            }
        })
        .count();
    dbg!(full_overlaps);
    Ok(())
}

fn part2() -> color_eyre::Result<()> {
    let full_overlaps = include_str!("../../data/day04/input.txt")
        .lines()
        .filter_map(|line| {
            let ranges = line.split_once(',')?;
            let first = ranges.0.split_once('-')?;
            let (x1, y1) = (
                first.0.parse::<u32>().unwrap(),
                first.1.parse::<u32>().unwrap(),
            );

            let second = ranges.1.split_once('-')?;
            let (x2, y2) = (
                second.0.parse::<u32>().unwrap(),
                second.1.parse::<u32>().unwrap(),
            );
            let is_overlapping = (x1 <= x2 && y1 >= x2)
                || (x1 <= y2 && y1 >= y2)
                || (x2 <= x1 && y2 >= x1)
                || (x2 <= y1 && y2 >= y1);
            dbg!((line, is_overlapping));

            if is_overlapping {
                Some(())
            } else {
                None
            }
        })
        .count();
    dbg!(full_overlaps);
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part() {
        assert_eq!(1, 1);
    }
}
