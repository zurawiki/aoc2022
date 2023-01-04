use clap::Parser;

/// Argument template for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify part to compute (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

use nom::{combinator::all_consuming, Finish};

use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{preceded, separated_pair},
    IResult,
};

use nom::{bytes::complete::take_while1, combinator::map};

fn get_input() -> &'static str {
    // include_str!("../../data/day07/example.txt")
    include_str!("../../data/day07/input.txt")
}

fn parse_path(i: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| c.is_alphabetic() || "./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(String);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}
#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(u64, String),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug)]
struct FsEntry {
    path: String,
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

fn part1() -> color_eyre::Result<()> {
    let lines = get_input()
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut stack = vec![FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // just ignore those
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    }
                    _ => {
                        let node = FsEntry {
                            path: path.clone(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, path) => {
                    let node = FsEntry {
                        size,
                        path,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }
    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }
    dbg!(&root);
    // solving part 1 because it's the same difficulty as part 2, just less code
    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    dbg!(sum);

    Ok(())
}

fn part2() -> color_eyre::Result<()> {
    let lines = get_input()
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut stack = vec![FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // just ignore those
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    }
                    _ => {
                        let node = FsEntry {
                            path: path.clone(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, path) => {
                    let node = FsEntry {
                        size,
                        path,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }
    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }
    dbg!(&root);

    let total_space = 70000000_u64;
    let used_space = root.total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_to_remove = root
        .all_dirs()
        .map(|n| n.total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();
    dbg!(size_to_remove);

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
