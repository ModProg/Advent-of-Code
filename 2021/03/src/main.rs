#[cfg(one)]
fn main() {
    const LINE_LEN: usize = 12;
    let input = include_str!("input")
        .lines()
        .fold([(0, 0); LINE_LEN], |mut acc, s| {
            s.chars().enumerate().for_each(|(i, c)| {
                if c == '1' {
                    acc[i].1 += 1
                } else {
                    acc[i].0 += 1
                }
            });
            acc
        })
        .into_iter()
        .rev()
        .enumerate()
        .fold((0, 0), |acc, (i, value)| {
            let value = if value.0 > value.1 { (1, 0) } else { (0, 1) };
            (acc.0 + (value.0 << i), acc.1 + (value.1 << i))
        });

    dbg!(input.0 * input.1);
}

fn bis_to_num(bits: &[u32]) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, value)| acc + (value << i))
}

// #[cfg(two)]
fn main() {
    let numbers: Vec<Vec<_>> = include_str!("input")
        .lines()
        .map(|s| s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect())
        .collect();

    let mut numbers_o = numbers.clone();
    let mut numbers_c = numbers;
    for i in 0..numbers_o[0].len() {
        let common = numbers_o.iter().fold((0, 0), |mut acc, v| {
            if v[i] == 0 {
                acc.0 += 1;
            } else {
                acc.1 += 1;
            }
            acc
        });
        numbers_o = numbers_o
            .into_iter()
            .filter(|v| {
                if common.0 > common.1 {
                    v[i] == 0
                } else {
                    v[i] == 1
                }
            })
            .collect();
        let common = numbers_c.iter().fold((0, 0), |mut acc, v| {
            if v[i] == 0 {
                acc.0 += 1;
            } else {
                acc.1 += 1;
            }
            acc
        });
        if numbers_c.len() > 1 {
            numbers_c = numbers_c
                .into_iter()
                .filter(|v| {
                    if common.0 <= common.1 {
                        v[i] == 0
                    } else {
                        v[i] == 1
                    }
                })
                .collect();
        }
    }
    dbg!(bis_to_num(&numbers_o[0]) * bis_to_num(&numbers_c[0]));
}
