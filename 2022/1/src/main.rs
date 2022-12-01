#![doc = include_str!("../puzzle.md")]
use std::{cmp::Reverse, env, str::FromStr};

use anyhow::{bail, Error, Result};
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

    let mut output = INPUT.lines().peekable().batching(|it| {
        it.peek().is_some().then(|| {
            let value = it
                .peeking_take_while(|line| !line.is_empty())
                .map(u32::from_str)
                .try_fold(0, |aggr, value| value.map(|value| aggr + value));
            it.next();
            value
        })
    });
    let output = match part {
        Part::One => output.try_fold(0, |aggr, value| value.map(|value| aggr.max(value)))?,
        Part::Two => output
            .try_fold(vec![0u32; 3], |mut aggr, value| {
                value.map(|value| {
                    aggr.push(value);
                    aggr.sort_by_key(|v| Reverse(*v));
                    aggr.pop();
                    aggr
                })
            })?
            .iter()
            .sum(),
    };

    println!("{output}");
    Ok(())
}
