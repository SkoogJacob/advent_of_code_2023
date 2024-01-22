use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

struct Game {
    id: usize,
    draws: Vec<Draw>,
}

struct Draw {
    colour_draw: HashMap<String, usize>
}

enum GameFromStrError {
    UnableToGetGameId(String),
    UnableToGetDraw(String),
    ParseDrawError(DrawFromStrError),
    ParseNumberError
}

enum DrawFromStrError {}
impl FromStr for Game {
    type Err = GameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, draw_info) = {
            let split: Vec<&str> = s.split(':').collect();
            let game_id = *match split.get(0) {
                None => return Err(GameFromStrError::UnableToGetGameId(
                    String::from("Shouldn't happen, string slice appears to have been empty")
                )),
                Some(s) => s
            };
            let game_id = match game_id.split(' ').collect::<Vec<&str>>().get(1) {
                None => return Err(GameFromStrError::UnableToGetGameId(
                    String::from("No space-separated game ID")
                )),
                Some(s) => {
                    match s.parse::<usize>() {
                        Ok(v) => {v}
                        Err(_) => {return Err(GameFromStrError::ParseNumberError)}
                    }
                }
            };
            let draw_info = match split.get(1) {
                None => return Err(GameFromStrError::UnableToGetDraw(
                    String::from("No draw info after the semicolon")
                )),
                Some(s) => s.to_string()
            };
            (game_id, draw_info)
        };

        let mut draws = draw_info.split(';')
            .map(|draw| Draw::from_str(draw));
        if draws.any(Result::is_err) {
            return Err(
                GameFromStrError::ParseDrawError(draws.find(Result::is_err).unwrap().unwrap_err())
            )
        }
        let draws = draws.map(Result::unwrap).collect::<Vec<Draw>>();

        Ok(Self { id, draws })
    }
}

impl FromStr for Draw {
    type Err = DrawFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}


fn main() {
    let input = {
        let args = std::env::args();
        let args: Vec<String> = args.collect();
        String::from(args.get(1).expect("no path to input given"))
    };
    let input = std::fs::read_to_string(&input);
}
