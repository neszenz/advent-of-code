use std::collections::HashMap;
use regex::Regex;

const RESOURCE_FILE_PATH: &str = "res/input";

fn main() {
    let input = std::fs::read_to_string(RESOURCE_FILE_PATH).expect("resource file can be loaded");

    let word_to_number: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    let some_number = Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let rebmun_emos = Regex::new(r"1|2|3|4|5|6|7|8|9|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let result: i32 = input
        .lines()
        .map(|line| {
            let first_match = some_number.find(line).unwrap().as_str();

            let enil = line.chars().rev().collect::<String>();
            let last_match = rebmun_emos.find(&enil).unwrap().as_str().chars().rev().collect::<String>();

            let first = if first_match.len() == 1 {
                first_match
            }
            else {
                word_to_number[first_match]
            };
            let last = if last_match.len() == 1 {
                last_match.as_str()
            }
            else {
                word_to_number[last_match.as_str()]
            };

            (first.to_string() + &last.to_string()).parse::<i32>().unwrap()
        })
        .sum();

    println!("result={}", result);
}
