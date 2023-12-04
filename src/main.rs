use std::vec::Vec;
use regex::Regex;
use lazy_static::lazy_static;

const RESOURCE_FILE_PATH: &str = "res/input";

lazy_static! {
    static ref GAME_RECORD_PATTERN: Regex = Regex::new(r".*\s(\d*):(.*)$").unwrap();
    static ref ENTRY_PATTERN: Regex = Regex::new(r"(\d*)\s*(\w*)$").unwrap();
}

enum Entry {
    Red(i32),
    Green(i32),
    Blue(i32),
}

impl From<&str> for Entry {
    fn from(entry_string: &str) -> Self {
        let captures = ENTRY_PATTERN.captures(entry_string).unwrap();
        let value = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let label = captures.get(2).unwrap().as_str();

        match label {
            "red" => Entry::Red(value),
            "green" => Entry::Green(value),
            "blue" => Entry::Blue(value),
            _ => panic!("label={} cannot be matched to either 'red', 'green' or 'blue'", label)
        }
    }
}

struct BallCount {
    n_red: i32,
    n_green: i32,
    n_blue: i32,
}

impl From<Vec<Entry>> for BallCount {
    fn from(entries: Vec<Entry>) -> Self {
        let mut red_entries: Vec<i32> = Vec::new();
        let mut green_entries: Vec<i32> = Vec::new();
        let mut blue_entries: Vec<i32> = Vec::new();

        for e in entries {
            match e {
                Entry::Red(value) => red_entries.push(value),
                Entry::Green(v) => green_entries.push(v),
                Entry::Blue(v) => blue_entries.push(v)
            }
        }

        BallCount {
            n_red: red_entries.iter().sum(),
            n_green: green_entries.iter().sum(),
            n_blue: blue_entries.iter().sum(),
        }
    }
}

struct Game {
    id: i32,
    requirements: BallCount,
}

impl Game {
    fn is_possible_for(self: &Game, given_balls: BallCount) -> bool {
        let enought_red = given_balls.n_red >= self.requirements.n_red;
        let enought_green = given_balls.n_green >= self.requirements.n_green;
        let enought_blue = given_balls.n_blue >= self.requirements.n_blue;

        enought_red && enought_green && enought_blue
    }
}

fn main() {
    let input = std::fs::read_to_string(RESOURCE_FILE_PATH).expect("resource file can be loaded");

    const GIVEN_LIMIT: BallCount = BallCount { n_red: 12, n_green: 13, n_blue: 14 };

    let mut games: Vec<Game> = Vec::new();

    let mut game_index: i32 = 0;
    for line in input.lines() {
        let c = GAME_RECORD_PATTERN.captures(line).unwrap();
        assert!(c.len() == 3);

        let game_id: i32 = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
        assert!(game_index + 1 == game_id);

        let game_requirements = c
            .get(2)
            .unwrap()
            .as_str()
            .split(';')
            .map(|item| item.trim())
            .map(|r| {
                let entries = r.split(',').map(|e| Entry::from(e)).collect::<Vec<Entry>>();
                BallCount::from(entries)
            })
            .reduce(|lhs, rhs|
                BallCount {
                    n_red: lhs.n_red.max(rhs.n_red),
                    n_green: lhs.n_green.max(rhs.n_green),
                    n_blue: lhs.n_blue.max(rhs.n_blue),
                }
            ).unwrap();

        games.push(Game{ id: game_id, requirements: game_requirements });

        game_index += 1;
    }

    let possible_games = games.iter().filter(|g| g.is_possible_for(GIVEN_LIMIT)).collect::<Vec<&Game>>();

    let result: i32 = possible_games.iter().map(|g| (*g).id).sum();

    println!("result={}", result);
}
