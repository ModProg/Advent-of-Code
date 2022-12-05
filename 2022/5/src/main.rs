#![feature(iter_next_chunk, array_chunks, array_try_map)]
#![doc = include_str!("../puzzle.md")]
use std::{env, mem, str::FromStr, vec};

use anyhow::{anyhow, bail, Context, Error, Result};

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

fn shared(
    input: &str,
    op: fn(count: usize, from: &mut Vec<u8>, to: &mut Vec<u8>) -> (),
) -> Result<String> {
    let width = input.find('\n').context("there is at least one line")? + 1;
    let height = (input.find('1').context("there is the label line")? - 1) / width;
    let mut stacks = vec![Vec::with_capacity(height); width / 4];
    for line in (0..height).rev() {
        for (idx, krate) in input[line * width..(line + 1) * width]
            .as_bytes()
            .array_chunks::<4>()
            .map(|&[_, c, ..]| c)
            .enumerate()
        {
            if krate != b' ' {
                stacks[idx].push(krate);
            }
        }
    }
    for instruction in input[((height + 1) * width + 1)..].lines() {
        let [_, count, _, from, _, to] = instruction
            .split(' ')
            .next_chunk()
            .map_err(|_| anyhow!("wrong number of tokens in instruction {instruction:?}"))?;
        let [count, from_idx, to_idx] = [count, from, to].try_map(usize::from_str)?;
        let from_idx = from_idx - 1;
        let to_idx = to_idx - 1;

        let mut from = mem::take(&mut stacks[from_idx]);
        let mut to = mem::take(&mut stacks[to_idx]);
        op(count, &mut from, &mut to);
        stacks[from_idx] = from;
        stacks[to_idx] = to;
    }
    stacks
        .iter_mut()
        .map(|stack| {
            stack
                .last()
                .context("empty stack")
                .map(|&byte| byte as char)
        })
        .collect()
}

fn one(input: &str) -> Result<String> {
    shared(input, |count, from, to| {
        to.extend(from.drain(from.len() - count..).rev());
    })
}

fn two(input: &str) -> Result<String> {
    shared(input, |count, from, to| {
        to.extend_from_slice(&from[from.len() - count..]);
        from.truncate(from.len() - count);
    })
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const INPUT: &str = indoc! {r#"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "#};

    #[test]
    fn one() {
        let output = "CMZ";
        assert_eq!(super::one(INPUT).unwrap(), output);
    }

    #[test]
    fn two() {
        let output = "MCD";
        assert_eq!(super::two(INPUT).unwrap(), output);
    }
}
