#![doc = include_str!("../puzzle.md")]
use std::{env, fmt::Display, iter, mem, str::FromStr};

use anyhow::{anyhow, bail, ensure, Context, Error, Result};

#[derive(Default, Debug)]
enum Part {
    #[default]
    One,
    Two,
}
impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::One,
            "2" => Self::Two,
            other => bail!("{other} is not a valid part"),
        })
    }
}
const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let part = env::args()
        .nth(1)
        .as_deref()
        .map(Part::from_str)
        .transpose()?
        .unwrap_or_default();

    eprintln!("=== Solving Part {part:?} ===");

    match part {
        Part::One => println!("{}", one(INPUT)?),
        Part::Two => println!("{}", two(INPUT)?),
    }

    Ok(())
}

#[derive(Debug)]
enum Line {
    Cd(&'static str),
    Ls,
    Dir(&'static str),
    File(u32),
    EoF,
}

impl TryFrom<&'static str> for Line {
    type Error = Error;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        Ok(if let Some(command) = value.strip_prefix("$ ") {
            if command == "ls" {
                Self::Ls
            } else if let Some(dir) = command.strip_prefix("cd ") {
                Self::Cd(dir)
            } else {
                bail!("unknown command {command}")
            }
        } else if let Some(dir) = value.strip_prefix("dir ") {
            Self::Dir(dir)
        } else {
            let (size, _name) = value.split_once(' ').context("parsing file entry")?;
            Self::File(size.parse().context("parsing file size")?)
        })
    }
}

fn shared(input: &'static str) -> Result<impl Iterator<Item = Result<u32>>> {
    let mut input = input.lines();
    ensure!(input.next() == Some("$ cd /"), "first command is cd /");
    let mut stack = Vec::<(&str, u32)>::new();
    let mut current_dir = "/";
    let mut current_sum = 0;
    Ok(input.map(Line::try_from).chain(iter::once(Ok(Line::EoF))).flat_map(move |line| -> Box<dyn Iterator<Item = Result<u32, Error>>>{
        if let Ok(line) = line {
            match line {
                Line::Cd("..") => {
                    let old_sum = current_sum;

                    let Some(parent) = stack.pop() else {return Box::new(iter::once(Err(anyhow!("Trying to go passt /"))))};
                    current_dir = parent.0;
                    current_sum += parent.1;

                    return Box::new(iter::once(Ok(old_sum)))
                }
                Line::Cd(dir) => {
                    stack.push((current_dir, current_sum));
                    current_dir = dir;
                    current_sum = 0;
                }
                Line::Ls | Line::Dir(_) => {},
                Line::File(file_size) =>  current_sum += file_size,
                Line::EoF => {
                    let mut current_sum = current_sum;
                    return Box::new(mem::take(&mut stack)
                        .into_iter()
                        .chain(iter::once(("/", 0)))
                        .map(move |(_,size)| {
                            let old_sum = current_sum;
                            current_sum += size;
                            anyhow::Ok(old_sum)
                        }))
                }
            }
                    Box::new(iter::empty())
        } else {
            Box::new(iter::once(Err(line.unwrap_err())))
        }
    }))
}

fn one(input: &'static str) -> Result<impl Display> {
    shared(input)?
        .filter(|size| {
            if let &Ok(size) = size {
                size < 100000
            } else {
                false
            }
        })
        .sum::<Result<u32>>()
}

fn two(input: &'static str) -> Result<impl Display> {
    let input = shared(input)?.collect::<Result<Vec<_>>>()?;
    let min_size = 30_000_000 - (70_000_000 - input.last().context("empty folder size array")?);
    dbg!(min_size);
    input
        .into_iter()
        .filter(|&dir| dir > min_size)
        .min()
        .context("no large enough folder")
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const INPUT: &str = indoc! {r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "#};

    #[test]
    fn one() {
        let output = "95437";
        assert_eq!(super::one(INPUT).unwrap().to_string(), output);
    }

    #[test]
    fn two() {
        let output = "24933642";
        assert_eq!(super::two(INPUT).unwrap().to_string(), output);
    }
}
