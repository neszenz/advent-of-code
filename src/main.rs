use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, HashMap};

const RESOURCE_FILE_PATH: &str = "res/input";

lazy_static!{
    static ref SCRATCHCARD_PATTERN: Regex = Regex::new(r"Card\s*(\d+):([\s0-9]*)\|([\s0-9]*)").unwrap();
}

struct Scratchcard {
    winning_numbers: HashSet<i32>,
    numbers: Vec<i32>,
}

impl From<&str> for Scratchcard {
    fn from(value: &str) -> Self {
        let caputes = SCRATCHCARD_PATTERN.captures(value).unwrap();
        assert!(caputes.len() == 4);

        let winning_numbers = HashSet::from_iter(caputes
            .get(2)
            .unwrap()
            .as_str()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap()));

        let numbers = caputes
            .get(3)
            .unwrap()
            .as_str()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Scratchcard { winning_numbers, numbers }
    }
}

impl Scratchcard {
    fn n_matches(self: &Self) -> usize {
         self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

fn main() {
    let input = std::fs::read_to_string(RESOURCE_FILE_PATH).expect("resource file can be loaded");

    let scratchcards = input
        .lines()
        .map(|l| Scratchcard::from(l))
        .collect::<Vec<Scratchcard>>();

    let result: usize = {
        let mut cache: HashMap<usize, usize> = HashMap::new();

        for i in (0..scratchcards.len()).rev() {
            assert!(!cache.contains_key(&i));

            let inherited_amout: usize = (0..scratchcards[i].n_matches())
                .map(|offset| {
                    cache.get(&(i+offset+1)).unwrap_or(&0)
                })
                .sum();

            cache.insert(i, 1 + inherited_amout);
        }

        cache.iter().map(|item| item.1).sum()
    };

    println!("result={}", result);
}
