#![feature(box_syntax)]
#![allow(clippy::type_complexity)]
#![doc = include_str!("../README.md")]
use std::{env, fmt::Display, mem, ops::Mul, str::FromStr};

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

struct Monkey {
    items: Vec<u64>,
    op: Box<dyn Fn(&mut u64)>,
    test: (Box<dyn Fn(u64) -> usize>, u64),
    interactions: u64,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.lines();
        s.next();
        Ok(Monkey {
            items: s
                .next()
                .and_then(|s| s.strip_prefix("  Starting items: "))
                .context("has starting items")?
                .split(", ")
                .map(u64::from_str)
                .collect::<Result<_, _>>()
                .context("starting items")?,
            op: s
                .next()
                .and_then(|s| s.strip_prefix("  Operation: new = old "))
                .and_then(|s| s.split_once(' '))
                .context("has operation")
                .and_then(|(op, value)| -> Result<Box<dyn Fn(&mut u64)>> {
                    Ok(if value == "old" {
                        match op {
                            "*" => box move |v| *v *= *v,
                            "+" => box move |v| *v += *v,
                            op => bail!("unsupported operation {op:?}"),
                        }
                    } else {
                        let value = u64::from_str(value)?;
                        match op {
                            "*" => box move |v| *v *= value,
                            "+" => box move |v| *v += value,
                            op => bail!("unsupported operation {op:?}"),
                        }
                    })
                })
                .context("operation")?,
            test: s
                .next()
                .and_then(|s| s.strip_prefix("  Test: divisible by "))
                .context("has test")
                .and_then(|divisor| u64::from_str(divisor).map_err(From::from))
                .and_then(|divisor| -> Result<(Box<dyn Fn(u64) -> usize>, u64)> {
                    let t = s
                        .next()
                        .and_then(|s| s.strip_prefix("    If true: throw to monkey "))
                        .context("has true case")
                        .and_then(|v| usize::from_str(v).map_err(From::from))?;
                    let f = s
                        .next()
                        .and_then(|s| s.strip_prefix("    If false: throw to monkey "))
                        .context("has true case")
                        .and_then(|v| usize::from_str(v).map_err(From::from))?;
                    Ok((box move |v| if v % divisor == 0 { t } else { f }, divisor))
                })
                .context("test")?,
            interactions: 0,
        })
    }
}

fn shared(input: &str, rounds: u64, devisor: u64) -> Result<impl Display> {
    let mut input = input
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>>>()?;
    let size_devisor = input.iter().fold(1, |aggr, monkey| aggr * monkey.test.1);
    for _ in 0..rounds {
        for monkey in 0..input.len() {
            let items = mem::take(&mut input[monkey].items);
            for mut item in items.into_iter().rev() {
                (input[monkey].op)(&mut item);
                item /= devisor;
                item %= size_devisor;
                input[monkey].interactions += 1;
                let monkey = (input[monkey].test.0)(item);
                input[monkey].items.push(item);
            }
        }
    }
    input
        .iter()
        .fold([0, 0], |aggr, curr| {
            let mut aggr = [curr.interactions, aggr[0], aggr[1]];
            aggr.sort_unstable();
            [aggr[1], aggr[2]]
        })
        .into_iter()
        .reduce(Mul::mul)
        .context("there are more than 2 monkeys")
}

fn one(input: &str) -> Result<impl Display> {
    shared(input, 20, 3)
}

fn two(input: &str) -> Result<impl Display> {
    shared(input, 10000, 1)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const INPUT: &str = indoc! {r#"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        
        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
        
        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
        
        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "#};

    #[test]
    fn one() {
        let output = "10605";
        assert_eq!(super::one(INPUT).unwrap().to_string(), output);
    }

    #[test]
    fn two() {
        let output = "2713310158";
        assert_eq!(super::two(INPUT).unwrap().to_string(), output);
    }
}
