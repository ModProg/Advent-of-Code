#![feature(iter_array_chunks)]
#![doc = include_str!("../README.md")]
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
                let (a, b) = line.split_at(line.len() / 2);
                let duplicate = a.chars().find(|a| b.contains(*a)).unwrap() as u8;
                (if duplicate >= b'a' {
                    duplicate - b'a'
                } else {
                    duplicate - b'A' + 26
                } + 1) as u32
            })
            .sum::<u32>(),
        Part::Two => INPUT
            .lines()
            .array_chunks::<3>()
            .map(|[a, b, c]| {
                let duplicate = a.chars().find(|&a| b.contains(a) && c.contains(a)).unwrap() as u8;
                (if duplicate >= b'a' {
                    duplicate - b'a'
                } else {
                    duplicate - b'A' + 26
                } + 1) as u32
            })
            .sum(),
    };

    println!("{output}");
    Ok(())
}
