fn is_numerical(c: char) -> bool {
    match c {
        '0' |
        '1' |
        '2' |
        '3' |
        '4' |
        '5' |
        '6' |
        '7' |
        '8' |
        '9' => true,
        _ => false
    }
}

fn convert_to_number(c1: char, c2: char) -> i32 {
    // b'0' = 48,
    // (c1 - N) * 10 + (c2 - N) = 10 * c1 - 10 * N + c2 - N = 10 * c1 + c2 - 11 * N
    c1 as i32 * 10 + c2 as i32 - 11 * b'0' as i32
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("resources/input.txt")?;
    let sum: i32 =input
    .lines()
    .map(|line| {
        let numbers: Vec<_> = line.chars().filter(|c| is_numerical(*c)).collect();
        match (numbers.first(), numbers.last()) {
            (None, None) |
            (None, Some(_)) |
            (Some(_), None) => 0,
            (Some(c1), Some(c2)) => convert_to_number(*c1, *c2)
        }
    }).sum();
    println!("The sum of the elves calibration nonsense is: {}", sum);
    Ok(())
}
