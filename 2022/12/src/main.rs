#![doc = include_str!("../README.md")]
use std::{env, fmt::Display, str::FromStr};

use anyhow::{bail, Error, Result};

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

fn one(input: &str) -> Result<impl Display> {
    Ok(todo!("Do part one") as &str)
}

fn two(input: &str) -> Result<impl Display> {
    Ok(todo!("Do part two") as &str)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const INPUT: &str = indoc! {r#"

    "#};

    #[test]
    fn one() {
        let output = "";
        assert_eq!(super::one(INPUT).unwrap().to_string(), output);
    }

    #[test]
    fn two() {
        let output = "";
        assert_eq!(super::two(INPUT).unwrap().to_string(), output);
    }
}
