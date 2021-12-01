use std::str::FromStr;

fn main() {
    let mut increasing = 0;
    let values = include_str!("../input")
        .lines()
        .map(|a| usize::from_str(a).unwrap());
    #[cfg(feature = "first")]
    let mut values = values.peekable();
    #[cfg(not(feature = "first"))]
    let values: Vec<_> = values.collect();

    #[cfg(feature = "first")]
    while let (Some(last), Some(&current)) = (values.next(), values.peek()) {
        if last < current {
            increasing += 1;
        }
    }

    for i in 1..(values.len() - 2) {
        if values[(i - 1)..(i + 2)].iter().sum::<usize>()
            < values[(i)..(i + 3)].iter().sum::<usize>()
        {
            increasing += 1;
        }
    }

    dbg!(increasing);
}
