#[cfg(feature = "one")]
fn main() {
    let mut x = 0;
    let mut y = 0;
    for (direction, value) in include_str!("input")
        .lines()
        .map(|s| s.split_once(' ').unwrap())
    {
        let value: usize = value.parse().unwrap();
        match direction {
            "forward" => x += value,
            "down" => y += value,
            "up" => y -= value,
            _ => unreachable!(),
        }
    }
    dbg!(x * y);
    println!("Hello, world!");
}

#[cfg(not(feature = "one"))]
fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (direction, value) in include_str!("input")
        .lines()
        .map(|s| s.split_once(' ').unwrap())
    {
        let value: usize = value.parse().unwrap();
        match direction {
            "forward" => {
                x += value;
                y += aim * value
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    }
    dbg!(x * y);
    println!("Hello, world!");
}
