#![doc = include_str!("../README.md")]
use std::{collections::HashSet, env, fmt::Display, str::FromStr};

use anyhow::{bail, Context, Error, Result};

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
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut rope = [(0i32, 0i32); N];
    for instruction in input.lines() {
        let (direction, steps) = instruction
            .split_once(' ')
            .context("instruction format is `<D> <S>`")?;
        for _ in 0..steps.parse()? {
            match direction {
                "U" => rope[0].1 += 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                "D" => rope[0].1 -= 1,
                other => bail!("Unsupported direction {other}"),
            }
            for idx in 1..N {
                let h = rope[idx - 1];
                let t = &mut rope[idx];
                if h.0 == t.0 && h.1.abs_diff(t.1) > 1 {
                    t.1 += (h.1 - t.1).signum();
                } else if h.1 == t.1 && h.0.abs_diff(t.0) > 1 {
                    t.0 += (h.0 - t.0).signum();
                } else if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                    t.0 += (h.0 - t.0).signum();
                    t.1 += (h.1 - t.1).signum();
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    Ok(visited.len())
}

fn one(input: &str) -> Result<impl Display> {
    shared::<2>(input)
}

fn two(input: &str) -> Result<impl Display> {
    shared::<10>(input)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    #[test]
    fn one() {
        const INPUT: &str = indoc! {r#"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "#};
        let output = "13";
        assert_eq!(super::one(INPUT).unwrap().to_string(), output);
    }

    #[test]
    fn two() {
        const INPUT: &str = indoc! {r#"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "#};
        let output = "36";
        assert_eq!(super::two(INPUT).unwrap().to_string(), output);
    }
}
