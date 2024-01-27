use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;
use std::error::Error;

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

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {}: ", self.id)?;
        let mut stream = self.draws.iter().map(|d| {
            write!(f, "{},", d)
        });
        if stream.any(|r| r.is_err()) {
            return stream.find(|r| r.is_err()).unwrap()
        }
        write!(f, ";;")
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

impl std::fmt::Display for Draw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut stream = self.colour_draw.keys().map(|key| {
            write!(f, "{}--{},", key, *self.colour_draw.get(key).unwrap())
        });
        if stream.any(|r| r.is_err()) {
            return stream.find(Result::is_err).unwrap()
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
enum GameFromStrError {
    UnableToGetGameId(String),
    UnableToGetDraw(String),
    ParseDrawError(DrawFromStrError),
    ParseNumberError
}

impl std::fmt::Display for GameFromStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameFromStrError::UnableToGetGameId(msg) => {
                write!(f, "Unable to get game id: {}", msg)
            }
            GameFromStrError::UnableToGetDraw(msg) => {
                write!(f, "Unable to get draw info: {}", msg)
            }
            GameFromStrError::ParseDrawError(_) => {
                write!(f, "Error while parsing draw")
            }
            GameFromStrError::ParseNumberError => {
                write!(f, "Couldn't parse number")
            }
        }
    }
}

impl Error for GameFromStrError {}

#[derive(Debug)]
enum DrawFromStrError {
    PlaceHolder
}
impl FromStr for Game {
    type Err = GameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, draw_info) = {
            let split: Vec<&str> = s.split(':').filter(|s| !s.is_empty()).collect();
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
            .filter(|s| !s.is_empty())
            .map(|draw| Draw::from_str(draw))
            .filter(Result::is_ok);
        let draws = draws.map(Result::unwrap).collect::<Vec<Draw>>();
        let game = Self { id, draws };
        Ok(game)
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
        bag
    };
    let mut ids: Vec<usize> = Vec::with_capacity(100);
    for line in input.lines() {
        let (id, draw_str) = {
            let parts = line.split(':')
                .filter(|s| !s.is_empty())
                .map(str::trim)
                .collect::<Box<[&str]>>();
            assert_eq!(parts.len(), 2);
            let id: usize = parts.get(0).unwrap()
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse().unwrap();
            let draw_info = *parts.get(1).unwrap();
            (id, draw_info)
        };
        if !draw_str.split(';')
            .map(|s| s.split(',').map(str::trim))
            .flatten()
            .any(|ball_info| {
                let parts = ball_info.split(' ').filter(|s| !s.is_empty()).collect::<Box<[&str]>>();
                assert_eq!(parts.len(), 2);
                let p1 = parts[0];
                let p2 = parts[1];
                if let Ok(n) = p1.parse::<usize>() {
                    n > *bag.get(p2).unwrap()
                } else if let Ok(n) = p2.parse::<usize>() {
                    n > *bag.get(p1).unwrap()
                } else {
                    false
                }
            }) {
            ids.push(id);
        }
    }
    println!("Sum of passed ids: {}", ids.iter().sum::<usize>());
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
    fn test1() {
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

    #[test]
    fn test2() {
        let mut bag = HashMap::new();
        bag.insert(String::from("red"), 12);
        bag.insert(String::from("green"), 13);
        bag.insert(String::from("blue"), 14);
        let bag = Draw::new(bag);
        // game 6: impossible
        // game 7: possible
        // game 27: impossible
        // game 39: impossible
        // game 40: possible
        // game 41: impossible
        // game 81: impossible
        // game 82: impossible
        // game 83: possible
        // expected sum: 7 + 40 + 83 = 130
        let games = r#"
Game 6: 6 red, 3 green, 6 blue; 3 green, 5 blue, 12 red; 3 green, 9 blue, 3 red; 13 red, 8 blue
Game 7: 3 blue, 1 red; 3 blue, 10 green; 4 green, 5 blue
Game 27: 4 blue, 15 green; 6 green, 2 blue, 1 red; 9 blue, 10 green, 4 red; 3 red, 3 green, 6 blue; 11 blue, 7 red, 11 green; 6 red, 5 green, 13 blue
Game 39: 3 green, 1 red, 4 blue; 9 green, 1 red, 18 blue; 4 red, 4 green, 17 blue; 4 red, 10 blue, 14 green
Game 40: 5 red, 4 green, 8 blue; 1 green, 9 blue; 9 blue, 3 red, 6 green; 8 red, 9 blue, 9 green
Game 41: 1 blue, 9 red, 3 green; 9 red, 10 green, 15 blue; 13 red, 8 green, 8 blue; 19 red, 6 blue, 2 green; 7 green, 5 blue, 12 red
Game 81: 1 green, 2 red, 11 blue; 5 red, 3 blue; 1 green, 1 red; 14 red, 1 green
Game 82: 12 red, 3 blue, 8 green; 15 red, 9 blue, 8 green; 6 blue, 13 red, 8 green
Game 83: 4 blue, 6 green, 3 red; 7 red, 2 blue, 9 green; 6 green, 3 red
        "#;
        let id_sum = games.lines().filter(|l| !l.is_empty())
            .map(|l| Game::from_str(l.trim()))
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|g| !g.impossible(&bag))
            .map(|g| g.id())
            .sum::<usize>();
        assert_eq!(id_sum, 130)
    }
}