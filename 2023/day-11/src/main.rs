#![allow(dead_code)]

const EMPTY_SPACE_SYMBOL: char = '.';
const GALAXY_SYMBOL: char = '#';

fn solve_part_1(input: &str) -> i32 {
    let y_len = input.lines().nth(0).unwrap().chars().count();
    assert!(input.lines().all(|line| line.chars().count() == y_len));

    let empty_row_indices: Vec<usize> = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|c| c == EMPTY_SPACE_SYMBOL))
        .map(|(x, _)| x)
        .collect();

    let empty_column_indices: Vec<usize> = (0..y_len)
        .filter(|y| input
            .lines()
            .all(|line| line.chars().nth(*y).unwrap() == EMPTY_SPACE_SYMBOL))
        .map(|y| y)
        .collect();

    fn correct_for_expansion(index: usize, empty_indices: &Vec<usize>) -> usize {
        index + empty_indices
            .iter()
            .filter(|empty_index| **empty_index < index)
            .count()
    }

    let galaxy_positions: Vec<(usize, usize)> = input
        .lines()
        .map(|line| line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == GALAXY_SYMBOL)
            .map(|(y, _)| correct_for_expansion(y, &empty_column_indices))
        )
        .enumerate()
        .flat_map(|(x, ys)| {
            let mut tmp = Vec::new();

            for y in ys {
                let new_pos = (correct_for_expansion(x, &empty_row_indices), y);
                tmp.push(new_pos)
            }

            tmp
        })
        .collect();

    let galaxy_pairs = (0..galaxy_positions.len()-1)
        .flat_map(|lhs_i| {
            let mut tmp = Vec::new();

            for rhs_i in lhs_i+1..galaxy_positions.len() {
                tmp.push((lhs_i, rhs_i));
            }

            tmp
        })
        .collect::<Vec<(usize, usize)>>();

    galaxy_pairs
        .iter()
        .map(|(lhs_i, rhs_i)| (galaxy_positions[*lhs_i], galaxy_positions[*rhs_i]))
        .map(|(lhs, rhs)| {
            let x_diff = (lhs.0 as i32 - rhs.0 as i32).abs();
            let y_diff = (lhs.1 as i32 - rhs.1 as i32).abs();

            x_diff + y_diff
        })
        .sum()
}

#[test]
fn example_1() {
    let result = solve_part_1(include_str!("../res/example_1"));
    assert_eq!(result, 374);
}
fn main() {
    let result = solve_part_1(include_str!("../res/input"));
    println!("result={result}");
}
