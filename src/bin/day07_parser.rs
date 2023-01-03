use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1, u64},
    combinator::{not, opt, peek},
    multi::many0,
    IResult,
};

#[derive(Eq, PartialEq)]
pub enum FsNode {
    Dir { name: String, listing: Vec<FsNode> },
    File { name: String, size: u64 },
}


impl FsNode {
  pub fn total_size(&self) -> u64 {
    match self {
      FsNode::File { size, .. } => *size,
      FsNode::Dir { listing, .. } => listing.iter().map(|n| n.total_size()).sum(),
    }
  }
}
impl fmt::Debug for FsNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsNode::File { name, size } => {
                write!(f, "file:{}|size:{}", name, size)
            }
            FsNode::Dir { name, listing } => {
                write!(f, "dir:{}|subfiles:{}", name, listing.len())
            }
        }
    }
}

impl<'a> IntoIterator for &'a FsNode {
    type Item = &'a FsNode;
    type IntoIter = std::vec::IntoIter<&'a FsNode>;

    fn into_iter(self) -> Self::IntoIter {
        fn append<'a>(tree: &'a FsNode, v: &mut Vec<&'a FsNode>) {
            match *tree {
                FsNode::File { .. } => {
                    v.push(tree);
                }
                FsNode::Dir { ref listing, .. } => {
                    v.push(tree);

                    for child in listing {
                        append(&child, v);
                    }
                }
            }
        }

        let mut result = vec![];
        append(self, &mut result);
        result.into_iter()
    }
}

pub fn ls_parse_file(input: &str) -> IResult<&str, Option<FsNode>> {
    let (input, size) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        Some(FsNode::File {
            name: name.to_string(),
            size,
        }),
    ))
}

#[test]
fn test_ls_parse_file() {
    assert_eq!(
        ls_parse_file("8504156 c.dat\n"),
        Ok((
            "",
            Some(FsNode::File {
                name: "c.dat".to_string(),
                size: 8504156,
            })
        ))
    );
}

pub fn ls_parse_dir(input: &str) -> IResult<&str, Option<FsNode>> {
    let (input, _) = tag("dir")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input, None, // we parse in the next cd,
    ))
}

#[test]
fn test_ls_parse_dir() {
    assert_eq!(ls_parse_dir("dir e\n"), Ok(("", None)));
}

pub fn ls_output(input: &str) -> IResult<&str, Vec<FsNode>> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, fs_node_list) = many0(alt((ls_parse_file, ls_parse_dir)))(input)?;
    Ok((input, fs_node_list.into_iter().flatten().collect()))
}

#[test]
fn test_ls_output() {
    assert_eq!(
        ls_output(
            r#"$ ls
"#
        ),
        Ok(("", vec![]))
    );
    assert_eq!(
        ls_output(
            r#"$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#
        ),
        Ok((
            "",
            vec![
                FsNode::File {
                    name: "j".to_string(),
                    size: 4060174,
                },
                FsNode::File {
                    name: "d.log".to_string(),
                    size: 8033020,
                },
                FsNode::File {
                    name: "d.ext".to_string(),
                    size: 5626152,
                },
                FsNode::File {
                    name: "k".to_string(),
                    size: 7214296,
                },
            ]
        ))
    );
}

pub fn cd_output(input: &str) -> IResult<&str, Vec<FsNode>> {
    let (input, _) = tag("$ cd ")(input)?;
    let _ = not(peek(tag("..")))(input)?;

    // and name is not ..
    let (input, name) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, listings) = many0(alt((ls_output, cd_output)))(input)?;
    let (input, _) = opt(tag("$ cd ..\n"))(input)?;

    let mut listing: Vec<FsNode> = listings.into_iter().flatten().collect();
    listing.sort_by_key(|root| match root {
        FsNode::File { name, .. } | FsNode::Dir { name, .. } => name.to_owned(),
    });

    Ok((
        input,
        vec![FsNode::Dir {
            name: name.to_string(),
            listing,
        }],
    ))
}

#[test]
fn test_cd_output() {
    assert_eq!(
        cd_output(
            r#"$ cd e
$ ls
584 i
$ cd ..
"#
        ),
        Ok((
            "",
            vec![FsNode::Dir {
                name: "e".to_string(),
                listing: vec![FsNode::File {
                    name: "i".to_string(),
                    size: 584,
                },],
            }]
        ))
    );
}
