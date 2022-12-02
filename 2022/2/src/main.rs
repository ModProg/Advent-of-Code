#![doc = include_str!("../puzzle.md")]
use std::{env, str::FromStr};

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

    let output = match part {
        Part::One => INPUT
            .lines()
            .map(|line| {
                let (opponent, me) = line.split_once(' ').expect("Every line contains `<A> <X>`");
                let opponent = opponent.as_bytes()[0] - b'A';
                let me = me.as_bytes()[0] - b'X';
                (me + 1
                    + if opponent == me {
                        3
                    } else if (opponent + 1) % 3 == me {
                        6
                    } else {
                        0
                    }) as u32
            })
            .sum::<u32>(),

        Part::Two => INPUT
            .lines()
            .map(|line| {
                let (opponent, me) = line.split_once(' ').expect("Every line contains `<A> <X>`");
                let opponent = opponent.as_bytes()[0] - b'A';
                let me = me.as_bytes()[0] - b'X';
                (me * 3 + 1 + (3 + opponent + me - 1) % 3) as u32
            })
            .sum::<u32>(),
    };

    println!("{output}");
    Ok(())
}
