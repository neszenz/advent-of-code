use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref INSTRUCTION_PATTERN_PART_1: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    static ref INSTRUCTION_PATTERN_PART_2: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
}

fn solve_part_1(input: &str) -> i32 {
    INSTRUCTION_PATTERN_PART_1
        .captures_iter(input)
        .map(|c| {
            let lhs = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let rhs = c.get(2).unwrap().as_str().parse::<i32>().unwrap();

            lhs * rhs
        })
        .sum()
}

fn solve_part_2(input: &str) -> i32 {
    let mut is_enalbed = true;

    INSTRUCTION_PATTERN_PART_2
        .captures_iter(input)
        .map(|c| {
            let c_as_str = c.get(0).unwrap().as_str();

            if c_as_str.starts_with("mul(") {
                assert_eq!(c.len(), 3);

                if !is_enalbed {
                    return 0;
                }

                let lhs = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let rhs = c.get(2).unwrap().as_str().parse::<i32>().unwrap();

                return lhs * rhs;
            }
            else if c_as_str == "do()" {
                is_enalbed = true;
            }
            else if c_as_str == "don't()" {
                is_enalbed = false;
            }

            0
        })
        .sum()
}

#[test]
fn example_part_1() {
    let result = solve_part_1(include_str!("../res/example_part_1"));
    assert_eq!(result, 161);
}

#[test]
fn example_part_2() {
    let result = solve_part_2(include_str!("../res/example_part_2"));
    assert_eq!(result, 48);
}

fn main() {
    let result_part_1 = solve_part_1(include_str!("../res/input"));
    let result_part_2 = solve_part_2(include_str!("../res/input"));
    println!("result_part_1={result_part_1} result_part_2={result_part_2}");
}
