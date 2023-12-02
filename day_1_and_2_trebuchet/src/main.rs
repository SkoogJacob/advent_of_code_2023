enum Tokens {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Tokens {
    fn tokenize(line: &'_ str) -> Vec<Tokens> {
        let mut parser = LineParser::new(line);
        todo!()
    }
}

impl Into<i32> for Tokens {
    fn into(self) -> i32 {
        match self {
            Tokens::Zero => 0,
            Tokens::One => 1,
            Tokens::Two => 2,
            Tokens::Three => 3,
            Tokens::Four => 4,
            Tokens::Five => 5,
            Tokens::Six => 6,
            Tokens::Seven => 7,
            Tokens::Eight => 8,
            Tokens::Nine => 9,
        }
    }
}

impl TryFrom<i32> for Tokens {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            _ => Err(())
        }
    }
}

struct LineParser<'line> {
    line: &'line str,
    cursor: usize
}

impl<'line> LineParser<'line> {
    fn new(line: &'line str) -> Self {
        Self {
            line,
            cursor: 0
        }
    }
}

fn is_numerical(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn convert_to_number(c1: Tokens, c2: Tokens) -> i32 {
    todo!()
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("resources/input.txt")?;
    input
        .lines()
        .map(|line| Tokens::tokenize(line))
        .for_each(|_lmao| ());
    Ok(())
}
