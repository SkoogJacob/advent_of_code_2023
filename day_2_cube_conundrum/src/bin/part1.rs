use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    pub fn impossible(&self, bag: &Draw) -> bool {
        self.draws.iter().any(|d| d.impossible(bag))
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
struct Draw {
    colour_draw: HashMap<String, usize>
}

impl Draw {
    #[inline]
    pub fn new(balls: HashMap<String, usize>) -> Self {
        Self { colour_draw: balls }
    }

    /// Returns true if `self` could not have been drawn from the passed "bag"
    pub fn impossible(&self, bag: &Draw) -> bool {
        bag.colour_draw.keys().any(|ball_key| {
            self.colour_draw.get(ball_key).unwrap_or(&0) > bag.colour_draw.get(ball_key).unwrap()
        })
    }
}

#[derive(Debug)]
enum GameFromStrError {
    UnableToGetGameId(String),
    UnableToGetDraw(String),
    ParseDrawError(DrawFromStrError),
    ParseNumberError
}

#[derive(Debug)]
enum DrawFromStrError {
    PlaceHolder
}
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

        let draws = draw_info.split(';')
            .map(|draw| Draw::from_str(draw))
            .filter(Result::is_ok);
        let draws = draws.map(Result::unwrap).collect::<Vec<Draw>>();

        Ok(Self { id, draws })
    }
}

impl FromStr for Draw {
    type Err = DrawFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colour_draw : HashMap<String, usize>= HashMap::new();
        s.split(',')
            .map(str::trim)
            .map(|balls| {
                let split = balls.split(' ').collect::<Vec<&str>>();
                match (split.get(0), split.get(1)) {
                    (Some(v1), Some(v2)) => Ok((*v1, *v2)),
                    _ => Err(DrawFromStrError::PlaceHolder)
                }
            })
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .for_each(|(s1, s2)| {
                let (number, colour) = match (usize::from_str(s1), usize::from_str(s2)) {
                    (Ok(n), Err(_)) => (n, s2.to_string()),
                    (Err(_), Ok(n)) => (n, s1.to_string()),
                    _ => (0, String::from("ERR"))
                };
                colour_draw.insert(colour, number);
        });

        Ok(Self { colour_draw })
    }
}

fn main() {
    let input = {
        let args = std::env::args();
        let args: Vec<String> = args.collect();
        String::from(args.get(1).expect("no path to input given"))
    };
    let input = std::fs::read_to_string(&input).expect("Unable to read file to string");
    let bag = {
        let mut bag = HashMap::new();
        bag.insert(String::from("red"), 12);
        bag.insert(String::from("green"), 13);
        bag.insert(String::from("blue"), 14);
        Draw::new(bag)
    };
    let games = input.lines()
        .map(|l| Game::from_str(l))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(|g| g.impossible(&bag))
        .map(|g| g.id())
        .sum::<usize>();
    println!("Sum of passed ids: {}", games);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bag_config() -> Draw {
        let mut bag = HashMap::new();
        bag.insert("red".to_string(), 12);
        bag.insert("green".to_string(), 13);
        bag.insert("blue".to_string(), 14);
        Draw::new(bag)
    }

    #[inline]
    fn game_1() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    }

    #[inline]
    fn game_2() -> &'static str {
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
    }

    #[inline]
    fn game_3() -> &'static str {
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
    }

    #[inline]
    fn game_4() -> &'static str {
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
    }

    #[inline]
    fn game_5() -> &'static str {
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    /// tests if game `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
    /// parses correctly and evaluates as possible with the reference "bag config"
    #[test]
    fn test() {
        let games = vec![
            Game::from_str(game_1()).unwrap(),
            Game::from_str(game_2()).unwrap(),
            Game::from_str(game_3()).unwrap(),
            Game::from_str(game_4()).unwrap(),
            Game::from_str(game_5()).unwrap()
        ];

        let sum = games
            .iter()
            .filter(|g| !g.impossible(&bag_config()))
            .map(|g| g.id)
            .sum::<usize>();
        assert_eq!(sum, 8);
    }
}