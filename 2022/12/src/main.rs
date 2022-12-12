#![feature(is_some_and)]
#![doc = include_str!("../README.md")]
use std::{env, fmt::Display, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use pathfinding::prelude::astar;

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

fn shared(input: &str, start: char, target: char, part_2: bool) -> Result<impl Display> {
    let width = input.find('\n').context("there is a linefeed")?;
    let pos = |x, y| x + y * (width + 1);
    let s = input.find(start).context("there is a start")?;
    let e = input.find(target).context("there is a target")?;
    let input = input.as_bytes();
    let s = (s % (width + 1), s / (width + 1));
    let e = (e % (width + 1), e / (width + 1));
    Ok(astar(
        &s,
        |&c @ (x, y)| {
            let cost = |(x, y)| match input[pos(x, y)] {
                b'S' => b'a',
                b'E' => b'z',
                c => c,
            };
            let valid = |x, y| {
                if part_2 {
                    cost(c) < cost((x, y)) + 2
                } else {
                    cost((x, y)) < cost(c) + 2
                }
            };
            let mut neighboors = Vec::new();
            if x != 0 && valid(x - 1, y) {
                neighboors.push(((x - 1, y), 1))
            }
            if y != 0 && valid(x, y - 1) {
                neighboors.push(((x, y - 1), 1))
            }
            if x != width - 1 && valid(x + 1, y) {
                neighboors.push(((x + 1, y), 1))
            }
            if pos(x, y + 1) < input.len() && valid(x, y + 1) {
                neighboors.push(((x, y + 1), 1))
            }
            neighboors
        },
        |&(x, y)| {
            if part_2 {
                0
            } else {
                x.abs_diff(e.0) + y.abs_diff(e.1)
            }
        },
        |&(x, y)| input[pos(x, y)] == target as u8,
    )
    .context("there is a path")?
    .1)
}

fn one(input: &str) -> Result<impl Display> {
    shared(input, 'S', 'E', false)
}

fn two(input: &str) -> Result<impl Display> {
    shared(input, 'E', 'a', true)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const INPUT: &str = indoc! {r#"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "#};

    #[test]
    fn one() {
        let output = "31";
        assert_eq!(super::one(INPUT).unwrap().to_string(), output);
    }

    #[test]
    fn two() {
        let output = "29";
        assert_eq!(super::two(INPUT).unwrap().to_string(), output);
    }
}
