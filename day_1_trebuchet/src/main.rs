const ZERO_FOUR_FIVE_NINE: usize = "zero".len();
const ONE_TWO_SIX: usize = "one".len();
const THREE_SEVEN_EIGHT: usize = "three".len();

#[derive(Debug, Clone, Copy)]
pub enum Token {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl Token {
    fn token_length(&self) -> usize {
        match self {
            Token::THREE | Token::SEVEN | Token::EIGHT => THREE_SEVEN_EIGHT,
            Token::ZERO | Token::FOUR | Token::FIVE | Token::NINE => ZERO_FOUR_FIVE_NINE,
            Token::ONE | Token::TWO | Token::SIX => ONE_TWO_SIX,
        }
    }
}

impl From<Token> for i32 {
    fn from(value: Token) -> Self {
        match value {
            Token::ZERO => 0,
            Token::ONE => 1,
            Token::TWO => 2,
            Token::THREE => 3,
            Token::FOUR => 4,
            Token::FIVE => 5,
            Token::SIX => 6,
            Token::SEVEN => 7,
            Token::EIGHT => 8,
            Token::NINE => 9,
        }
    }
}

impl TryFrom<i32> for Token {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Token::ZERO,
            1 => Token::ONE,
            2 => Token::TWO,
            3 => Token::THREE,
            4 => Token::FOUR,
            5 => Token::FIVE,
            6 => Token::SIX,
            7 => Token::SEVEN,
            8 => Token::EIGHT,
            9 => Token::NINE,
            _ => {
                return Err(format!("Value {} is not in range 0-9", value));
            }
        })
    }
}

impl TryFrom<&'_ str> for Token {
    type Error = ();

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        match value {
            "zero" => Ok(Token::ZERO),
            "one" => Ok(Token::ONE),
            "two" => Ok(Token::TWO),
            "three" => Ok(Token::THREE),
            "four" => Ok(Token::FOUR),
            "five" => Ok(Token::FIVE),
            "six" => Ok(Token::SIX),
            "seven" => Ok(Token::SEVEN),
            "eight" => Ok(Token::EIGHT),
            "nine" => Ok(Token::NINE),
            _ => Err(()),
        }
    }
}

pub fn tokenize(line: &'_ str) -> Option<(Token, Token)> {
    fn update_token_and_cursor(
        token: Token,
        option_ref: &mut Option<Token>,
        cursor: &mut usize,
        increase: bool,
    ) {
        if increase {
            *cursor += token.token_length().saturating_sub(1)
        } else {
            *cursor = cursor.saturating_sub(token.token_length() + 1)
        }
        *option_ref = Some(token);
    }
    let mut l_cursor: usize = 0;

    let mut l_token: Option<Token> = None;
    let mut r_token: Option<Token> = None;
    while l_cursor < line.len() {
        match line
            .get(l_cursor..l_cursor + 1)
            .expect("Unable to get character on forwards search")
        {
            "0" => l_token = Some(Token::ZERO),
            "1" => l_token = Some(Token::ONE),
            "2" => l_token = Some(Token::TWO),
            "3" => l_token = Some(Token::THREE),
            "4" => l_token = Some(Token::FOUR),
            "5" => l_token = Some(Token::FIVE),
            "6" => l_token = Some(Token::SIX),
            "7" => l_token = Some(Token::SEVEN),
            "8" => l_token = Some(Token::EIGHT),
            "9" => l_token = Some(Token::NINE),
            "z" | "o" | "t" | "f" | "s" | "e" | "n" => {
                if let Some(number) = line.get(l_cursor..l_cursor + THREE_SEVEN_EIGHT) {
                    if let Ok(token) = Token::try_from(number) {
                        update_token_and_cursor(token, &mut l_token, &mut l_cursor, true);
                    } else if let Ok(token) = Token::try_from(&number[0..ZERO_FOUR_FIVE_NINE]) {
                        update_token_and_cursor(token, &mut l_token, &mut l_cursor, true);
                    } else if let Ok(token) = Token::try_from(&number[0..ONE_TWO_SIX]) {
                        update_token_and_cursor(token, &mut l_token, &mut l_cursor, true);
                    }
                } else if let Some(number) = line.get(l_cursor..l_cursor + ZERO_FOUR_FIVE_NINE) {
                    if let Ok(token) = Token::try_from(number) {
                        update_token_and_cursor(token, &mut l_token, &mut l_cursor, true);
                    } else if let Ok(token) = Token::try_from(&number[0..ONE_TWO_SIX]) {
                        update_token_and_cursor(token, &mut l_token, &mut l_cursor, true);
                    }
                } else if let Some(number) = line.get(l_cursor..l_cursor + ONE_TWO_SIX) {
                    if let Ok(token) = Token::try_from(number) {
                        update_token_and_cursor(token, &mut l_token, &mut l_cursor, true);
                    }
                }
            }
            _ => (),
        }
        l_cursor += 1;
        if l_token.is_some() {
            break;
        }
    }

    let line = &line[l_cursor..];
    let mut r_cursor = line.len();
    while r_cursor > 0 {
        match line
            .get(r_cursor.saturating_sub(1)..r_cursor)
            .expect("Unable to get character on backwards search")
        {
            "0" => r_token = Some(Token::ZERO),
            "1" => r_token = Some(Token::ONE),
            "2" => r_token = Some(Token::TWO),
            "3" => r_token = Some(Token::THREE),
            "4" => r_token = Some(Token::FOUR),
            "5" => r_token = Some(Token::FIVE),
            "6" => r_token = Some(Token::SIX),
            "7" => r_token = Some(Token::SEVEN),
            "8" => r_token = Some(Token::EIGHT),
            "9" => r_token = Some(Token::NINE),
            "o" | "e" | "r" | "x" | "n" | "t" => {
                if let Some(number) = line.get(r_cursor.wrapping_sub(THREE_SEVEN_EIGHT)..r_cursor) {
                    if let Ok(token) = Token::try_from(number) {
                        update_token_and_cursor(token, &mut r_token, &mut r_cursor, false);
                    } else if let Ok(token) =
                        Token::try_from(&number[number.len().wrapping_sub(ZERO_FOUR_FIVE_NINE)..])
                    {
                        update_token_and_cursor(token, &mut r_token, &mut r_cursor, false);
                    } else if let Ok(token) =
                        Token::try_from(&number[number.len().wrapping_sub(ONE_TWO_SIX)..])
                    {
                        update_token_and_cursor(token, &mut r_token, &mut r_cursor, false);
                    }
                } else if let Some(number) =
                    line.get(r_cursor.wrapping_sub(ZERO_FOUR_FIVE_NINE)..r_cursor)
                {
                    if let Ok(token) = Token::try_from(number) {
                        update_token_and_cursor(token, &mut r_token, &mut r_cursor, false);
                    } else if let Ok(token) =
                        Token::try_from(&number[number.len().wrapping_sub(ONE_TWO_SIX)..])
                    {
                        update_token_and_cursor(token, &mut r_token, &mut r_cursor, false);
                    }
                } else if let Some(number) = line.get(r_cursor.wrapping_sub(ONE_TWO_SIX)..r_cursor)
                {
                    if let Ok(token) = Token::try_from(number) {
                        update_token_and_cursor(token, &mut r_token, &mut r_cursor, false);
                    }
                }
            }
            _ => (),
        }
        r_cursor = r_cursor.saturating_sub(1);
        if r_token.is_some() {
            break;
        }
    }

    match (l_token, r_token) {
        (Some(l), Some(r)) => Some((l, r)),
        (Some(l), None) => Some((l, l)),
        _ => None,
    }
}

fn tokens_to_number(l_token: Token, r_token: Token) -> i32 {
    let l_token: i32 = l_token.into();
    let r_token: i32 = r_token.into();
    l_token * 10 + r_token
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("resources/input.txt")?;
    let sum: i32 = input
        .lines()
        .map(|line| {
            let tokens = tokenize(&line);
            if let Some((t1, t2)) = tokens {
                tokens_to_number(t1, t2)
            } else {
                0
            }
        })
        .sum();
    println!("Sum: {}", sum);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn convert_line_to_sum(line: &str) -> i32 {
        if let Some((t1, t2)) = tokenize(line) {
            tokens_to_number(t1, t2)
        } else {
            -1
        }
    }

    #[test]
    fn test_simple_line() {
        let line = "1gjdlkfgjsjfsljglkjfbvlsdfvjlskdfjglskdjfg6";
        if let Some((t1, t2)) = tokenize(line) {
            assert_eq!(tokens_to_number(t1, t2), 16);
        } else {
            panic!("Failed to parse line!")
        }
    }

    #[test]
    fn test_harder_line() {
        let line = "onegjdlkfgjsjfsljglkjfbvlsdfvjlskdfjglskdjfgsix";
        if let Some((t1, t2)) = tokenize(line) {
            assert_eq!(tokens_to_number(t1, t2), 16);
        } else {
            panic!("Failed to parse line!")
        }
    }

    #[test]
    fn test_harderer_line() {
        let line = "oooooooooonegjdlkfgjsjfsljglkjfbvlsdfvjlskdfjglskdjfgsevenxxxxxxxxxxx";
        if let Some((t1, t2)) = tokenize(line) {
            assert_eq!(tokens_to_number(t1, t2), 17);
        } else {
            panic!("Failed to parse line!")
        }
    }

    #[test]
    fn test_toy_input() {
        let l0 = "two1nine";
        let l1 = "eightwothree";
        let l2 = "abcone2threexyz";
        let l3 = "xtwone3four";
        let l4 = "4nineeightseven2";
        let l5 = "zoneight234";
        let l6 = "7pqrstsixteen";
        assert_eq!(convert_line_to_sum(l0), 29);
        assert_eq!(convert_line_to_sum(l1), 83);
        assert_eq!(convert_line_to_sum(l2), 13);
        assert_eq!(convert_line_to_sum(l3), 24);
        assert_eq!(convert_line_to_sum(l4), 42);
        assert_eq!(convert_line_to_sum(l5), 14);
        assert_eq!(convert_line_to_sum(l6), 76);
    }
}
