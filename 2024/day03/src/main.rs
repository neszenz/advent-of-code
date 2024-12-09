#![allow(unused)]

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref INSTRUCTION_PATTERN: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
}

fn solve(input: &str) -> i32 {
    INSTRUCTION_PATTERN
        .captures_iter(input)
        .map(|c| {
            let lhs = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let rhs = c.get(2).unwrap().as_str().parse::<i32>().unwrap();

            lhs * rhs
        })
        .sum()
}

#[test]
fn example() {
    let result = solve(include_str!("../res/example"));
    assert_eq!(result, 161);
}

fn main() {
    let result = solve(include_str!("../res/input"));
    println!("result={result}");
}
