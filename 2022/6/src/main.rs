#![feature(array_windows, slice_partition_dedup)]
#![doc = include_str!("../README.md")]
use std::{env, fmt::Display, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

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

fn shared<const N: usize>(input: &str) -> Result<impl Display> {
    let position = input
        .as_bytes()
        .array_windows::<N>()
        .find_position(|&&window| {
            let mut window = window;
            window.sort_unstable();
            window.partition_dedup().1.is_empty()
        })
        .context("There should be a signal start")?
        .0;
    Ok(position + N)
}

fn one(input: &str) -> Result<impl Display> {
    shared::<4>(input)
}

fn two(input: &str) -> Result<impl Display> {
    shared::<14>(input)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const INPUT: &str = indoc! {r#"
        mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#};

    #[test]
    fn one() {
        let output = "7";
        assert_eq!(super::one(INPUT).unwrap().to_string(), output);
    }

    #[test]
    fn two() {
        let output = "19";
        assert_eq!(super::two(INPUT).unwrap().to_string(), output);
    }
}
