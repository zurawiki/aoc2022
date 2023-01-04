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
    // include_str!("../../data/day08/example.txt")
    include_str!("../../data/day08/input.txt")
}

fn part1() -> color_eyre::Result<()> {
    let lines: Vec<Vec<u8>> = get_input()
        .lines()
        .map(|line| {
            line.bytes()
                .filter(|b| b.is_ascii_digit())
                .map(|b| b - b'0')
                .collect()
        })
        .collect::<Vec<_>>();
    dbg!(&lines);
    let width = lines.get(0).unwrap().len();
    let height = lines.len();
    dbg!((width, height));

    let mut visible_trees = lines
        .iter()
        .map(|line| line.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..width {
        let mut max: Option<u8> = None;
        for j in 0..height {
            let n = lines[i][j];
            if max.map_or_else(|| true, |max| n > max) {
                visible_trees[i][j] = true;
                max = Some(n);
            }
        }

        let mut max: Option<u8> = None;
        for j in (0..height).rev() {
            let n = lines[i][j];
            if max.map_or_else(|| true, |max| n > max) {
                visible_trees[i][j] = true;
                max = Some(n);
            }
        }
    }

    for j in 0..height {
        let mut max: Option<u8> = None;
        for i in 0..width {
            let n = lines[i][j];
            if max.map_or_else(|| true, |max| n > max) {
                visible_trees[i][j] = true;
                max = Some(n);
            }
        }

        let mut max: Option<u8> = None;
        for i in (0..width).rev() {
            let n = lines[i][j];
            if max.map_or_else(|| true, |max| n > max) {
                visible_trees[i][j] = true;
                max = Some(n);
            }
        }
    }

    dbg!(visible_trees.iter().flatten().filter(|b| **b).count());
    Ok(())
}

fn count_view<'a>(values: impl Iterator<Item = &'a u8> + Clone, height: u8) -> usize {
    let length = values.clone().count();

    let count = values
        .clone()
        .position(|&n| n >= height)
        .map(|i| i + 1)
        .unwrap_or(length);

    count
}

fn score_scenery(lines: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    let z_height = lines[i][j];

    let north = count_view(lines[..i].iter().map(|line| &line[j]).rev(), z_height);
    let south = count_view(lines[i + 1..].iter().map(|line| &line[j]), z_height);
    let west = count_view(lines[i][..j].iter().rev(), z_height);
    let east = count_view(lines[i][j + 1..].iter(), z_height);

    north * south * west * east
}

fn part2() -> color_eyre::Result<()> {
    let lines: Vec<Vec<u8>> = get_input()
        .lines()
        .map(|line| {
            line.bytes()
                .filter(|b| b.is_ascii_digit())
                .map(|b| b - b'0')
                .collect()
        })
        .collect::<Vec<_>>();
    dbg!(&lines);
    let width = lines.get(0).unwrap().len();
    let height = lines.len();
    dbg!((width, height));

    let mut max: Option<usize> = None;

    for i in 0..width {
        for j in 0..height {
            max = max.max(Some(score_scenery(&lines, i, j)));
        }
    }
    dbg!(max);
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
