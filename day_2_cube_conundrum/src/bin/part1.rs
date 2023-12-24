use std::str::FromStr;

struct Game {
    id: usize,
    draws: Vec<Draw>,
}

struct Draw(u8, u8, u8);

impl FromStr for Game {
    type Err = GameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(GameFromStrError::EmptyStrError);
        } else if s.contains('\n') {
            return Err(GameFromStrError::MultipleLinesError);
        }
        let (game_id, draws) = {
            let colon_split = s.split(':').collect::<Vec<&str>>();
            let id: String = colon_split
                .get(0)
                .expect("string appears to be empty")
                .chars()
                .filter(|c| c.is_numeric())
                .collect();
            (
                id.parse::<usize>(),
                colon_split.get(1).unwrap_or(&"").to_string(),
            )
        };

        let id = match game_id {
            Ok(v) => v,
            Err(e) => return Err(GameFromStrError::GameIdParseError(e)),
        };
        let draws: Vec<Draw> = draws
            .split(';')
            .map(|draw| match Draw::from_str(draw) {
                Ok(draw) => draw,
                Err(e) => return GameFromStrError::DrawError(e),
            })
            .collect();

        Ok(Self { id, draws })
    }
}

impl FromStr for Draw {
    type Err = DrawFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug)]
enum GameFromStrError {
    MultipleLinesError,
    EmptyStrError,
    DrawError(DrawFromStrError),
    GameIdParseError(std::num::ParseIntError),
}

#[derive(Debug)]
enum DrawFromStrError {}

fn main() {
    let input = {
        let args = std::env::args();
        let args: Vec<String> = args.collect();
        String::from(args.get(1).expect("no path to input given"))
    };
    let input = std::fs::read_to_string(&input);
}
