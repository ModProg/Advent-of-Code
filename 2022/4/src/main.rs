#![doc = include_str!("../README.md")]
use std::{env, str::FromStr};

use anyhow::{bail, Error, Result};
use range_ext::intersect::{Intersect, Intersection};

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

    let input = INPUT.lines().map(|line| {
        let (a, b) = line.split_once(',').unwrap();
        let (a_f, a_t) = a.split_once('-').unwrap();
        let (b_f, b_t) = b.split_once('-').unwrap();

        let [a_f, a_t, b_f, b_t] = [a_f, a_t, b_f, b_t].map(|n| u32::from_str(n).unwrap());
        (a_f..a_t + 1, b_f..b_t + 1)
    });

    let output = match part {
        Part::One => input
            .filter(|(a, b)| {
                matches!(
                    a.intersect(b),
                    Intersection::Same | Intersection::Over | Intersection::Within
                )
            })
            .count(),
        Part::Two => input.filter(|(a, b)| a.intersect(b).is_any()).count(),
    };

    println!("{output}");
    Ok(())
}
