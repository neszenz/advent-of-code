#![allow(dead_code)]

use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref ENTRY_PATTERN: Regex = Regex::new(r"(\w*)\s*=\s*\((\w*),\s*(\w*)\)").unwrap();
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

struct InstructionsAndNetwork {
    instructions: Vec<Instruction>,
    network: HashMap<String, Node>,
}

impl InstructionsAndNetwork {
    fn parse(input: &str) -> Self {
        let instructions_input = input.lines().nth(0).unwrap();
        assert!(input.lines().nth(1).unwrap().is_empty());
        let network_input = input.lines().skip(2).collect::<Vec<&str>>();

        let instructions: Vec<Instruction> = instructions_input
            .chars()
            .map(|c| match c{
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                unknown => panic!("unknown instruction char {unknown}"),
            })
            .collect();

        let network: HashMap<String, Node> = network_input
            .iter()
            .map(|entry| {
                let captures = ENTRY_PATTERN.captures(*entry).unwrap();
                assert_eq!(captures.len(), 4);

                let id = captures.get(1).unwrap().as_str().to_string();
                let left = captures.get(2).unwrap().as_str().to_string();
                let right = captures.get(3).unwrap().as_str().to_string();

                (id, Node { left, right })
            })
            .collect();

        InstructionsAndNetwork { instructions, network }
    }

    fn solve_part_1(Self { instructions, network }: &Self) -> usize {
        let mut step_counter: usize = 0;
        let mut curr_node_ids: Vec<&String> = network
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect();

        while !curr_node_ids.iter().all(|id| id.ends_with("Z")) {
            let instruction_index = step_counter % instructions.len();
            let instruction = &instructions[instruction_index];
            let nodes: Vec<&Node> = curr_node_ids
                .iter()
                .map(|id| network.get(*id).unwrap())
                .collect();

            let next_node_ids: Vec<&String> = nodes
                .iter()
                .map(|node| {
                    match instruction {
                        Instruction::Left => &node.left,
                        Instruction::Right => &node.right,
                    }
                })
                .collect();

            curr_node_ids = next_node_ids;
            step_counter += 1;
        }

        step_counter
    }

    fn solve_part_2(Self{ instructions, network }: &Self) -> usize {
        let starting_ids: Vec<&String> = network
            .keys()
            .filter(|k| k.ends_with("A"))
            .collect();

        let intervals: Vec<usize> = starting_ids
            .iter()
            .map(|id| {
                let mut ends_at: Vec<usize> = Vec::new();

                let mut curr_id = *id;
                let mut step_counter: usize = 0;
                loop {
                    if curr_id.ends_with("Z") {
                        ends_at.push(step_counter as usize);
                    }

                    if ends_at.len() >= 10 {
                        break;
                    }

                    let curr_node = &network[curr_id];

                    let curr_instruction = &instructions[step_counter % instructions.len()];
                    let next_id = match *curr_instruction {
                        Instruction::Left => &curr_node.left,
                        Instruction::Right => &curr_node.right,
                    };

                    curr_id = next_id;
                    step_counter += 1;
                }

                let diffs: Vec<usize> = ends_at.windows(2).map(|w| w[1] - w[0]).collect();
                assert_eq!(diffs.len(), 9);
                assert!(diffs.windows(2).all(|w| w[0] == w[1]));

                diffs[0]
            })
            .collect();

        let least_common_multiple = intervals
            .iter()
            .map(|i| num::integer::lcm(*i, *i))
            .reduce(|lhs, rhs| {
                num::integer::lcm(lhs, rhs)
            })
            .unwrap();

        least_common_multiple
    }
}

#[test]
fn example_1() {
    static EXAMPLE_INPUT: &str = include_str!("../res/example_1");
    static EXAMPLE_ANSWER: usize = 2;

    let data = InstructionsAndNetwork::parse(EXAMPLE_INPUT);

    let result = InstructionsAndNetwork::solve_part_1(&data);
    assert_eq!(result, EXAMPLE_ANSWER);
}

#[test]
fn example_2() {
    static EXAMPLE_INPUT: &str = include_str!("../res/example_2");
    static EXAMPLE_ANSWER: usize = 6;

    let data = InstructionsAndNetwork::parse(EXAMPLE_INPUT);

    let result = InstructionsAndNetwork::solve_part_1(&data);
    assert_eq!(result, EXAMPLE_ANSWER);
}

#[test]
fn example_3() {
    static EXAMPLE_INPUT: &str = include_str!("../res/example_3");
    static EXAMPLE_ANSWER: usize = 6;

    let data = InstructionsAndNetwork::parse(EXAMPLE_INPUT);

    let result = InstructionsAndNetwork::solve_part_2(&data);
    assert_eq!(result, EXAMPLE_ANSWER);
}

fn main() {
    static INPUT: &str = include_str!("../res/input");

    let data = InstructionsAndNetwork::parse(INPUT);

    let result = InstructionsAndNetwork::solve_part_2(&data);
    println!("result={result}");
}
