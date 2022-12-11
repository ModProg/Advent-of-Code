#![feature(is_some_and)]
#![doc = include_str!("../README.md")]
use std::{env, fmt::Display, ops::ControlFlow, str::FromStr};

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

trait Value<T> {
    fn value(self) -> T;
}

impl<T> Value<T> for ControlFlow<T, T> {
    fn value(self) -> T {
        match self {
            ControlFlow::Continue(value) | ControlFlow::Break(value) => value,
        }
    }
}

enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

fn find_first_idx<T: PartialOrd>(
    grid: &[Vec<T>],
    range: impl Iterator<Item = T>,
    direction: Direction,
) -> Vec<Vec<Option<usize>>> {
    let width = match direction {
        Direction::Top | Direction::Bottom => grid[0].len(),
        Direction::Left | Direction::Right => grid.len(),
    };
    range
        .map(move |height| {
            (0..grid.len())
                .try_fold(vec![None; width], |mut aggr, row| {
                    let mut filled = 0;
                    for col in 0..width {
                        if aggr[col].is_some() {
                            filled += 1;
                        } else if match direction {
                            Direction::Top => &grid[row][col],
                            Direction::Left => &grid[col][row],
                            Direction::Right => &grid[col][grid[0].len() - row - 1],
                            Direction::Bottom => &grid[grid.len() - row - 1][col],
                        } >= &height
                        {
                            aggr[col] = row.into();
                        }
                    }
                    if filled == width {
                        ControlFlow::Break(aggr)
                    } else {
                        ControlFlow::Continue(aggr)
                    }
                })
                .value()
        })
        .collect()
}

fn one(input: &str) -> Result<impl Display> {
    let input = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = input[0].len() - 1;
    let height = input.len() - 1;

    let top = &find_first_idx(&input, 0..=9, Direction::Top);
    let left = &find_first_idx(&input, 0..=9, Direction::Left);
    let right = &find_first_idx(&input, 0..=9, Direction::Right);
    let bottom = &find_first_idx(&input, 0..=9, Direction::Bottom);

    Ok(input
        .into_iter()
        .enumerate()
        .flat_map(|(row, value)| {
            value.into_iter().enumerate().filter(move |&(col, value)| {
                !(top[value as usize][col].is_some_and(|r| r < row)
                    && left[value as usize][row].is_some_and(|c| c < col)
                    && right[value as usize][row].is_some_and(|c| c < (width - col))
                    && bottom[value as usize][col].is_some_and(|r| r < (height - row)))
            })
        })
        .count())
}

fn two(input: &str) -> Result<impl Display> {
    let input = &input
        .lines()
        .map(|line| line.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    input[..input.len() - 1]
        .iter()
        .enumerate()
        .skip(1)
        .flat_map(|(row, value)| {
            value[..value.len() - 1]
                .iter()
                .copied()
                .enumerate()
                .skip(1)
                .map(move |(col, value)| {
                    ((0..row)
                        .rev()
                        .position(|row| input[row][col] >= value)
                        .unwrap_or(row - 1)
                        + 1)
                        * ((row + 1..input.len())
                            .position(|row| input[row][col] >= value)
                            .unwrap_or(input.len() - row - 2)
                            + 1)
                        * ((0..col)
                            .rev()
                            .position(|col| input[row][col] >= value)
                            .unwrap_or(col - 1)
                            + 1)
                        * ((col + 1..input[0].len())
                            .position(|col| input[row][col] >= value)
                            .unwrap_or(input[0].len() - col - 2)
                            + 1)
                })
        })
        .max()
        .context("there is one tree")
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const INPUT: &str = indoc! {r#"
        30373
        25512
        65332
        33549
        35390
    "#};

    #[test]
    fn one() {
        let output = "21";
        assert_eq!(super::one(INPUT).unwrap().to_string(), output);
    }

    #[test]
    fn two() {
        let output = "8";
        assert_eq!(super::two(INPUT).unwrap().to_string(), output);
    }
}
