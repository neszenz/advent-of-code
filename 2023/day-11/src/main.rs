const EMPTY_SPACE_SYMBOL: char = '.';
const GALAXY_SYMBOL: char = '#';

fn expension_corrected_index(index: usize, empty_indices: &Vec<usize>, expension_factor: usize) -> usize {
    index + (expension_factor - 1) * empty_indices
        .iter()
        .filter(|empty_index| **empty_index < index)
        .count()
}

struct GalaxyPositions (Vec<(usize, usize)>);

impl GalaxyPositions {
    fn parse(input: &str, expension_factor: usize) -> Self {
        let y_len = input.lines().nth(0).unwrap().chars().count();
        assert!(input.lines().all(|line| line.chars().count() == y_len));

        let empty_rows: Vec<usize> = input
            .lines()
            .enumerate()
            .filter(|(_, line)| line.chars().all(|c| c == EMPTY_SPACE_SYMBOL))
            .map(|(x, _)| x)
            .collect();

        let empty_columns: Vec<usize> = (0..y_len)
            .filter(|y| input
                .lines()
                .all(|line| line.chars().nth(*y).unwrap() == EMPTY_SPACE_SYMBOL))
            .map(|y| y)
            .collect();

        let positions: Vec<(usize, usize)> = input
            .lines()
            .map(|line| line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == GALAXY_SYMBOL)
                .map(|(y, _)| expension_corrected_index(y, &empty_columns, expension_factor))
            )
            .enumerate()
            .flat_map(|(x, ys)| {
                let mut tmp = Vec::new();

                for y in ys {
                    let new_pos = (expension_corrected_index(x, &empty_rows, expension_factor), y);
                    tmp.push(new_pos)
                }

                tmp
            })
            .collect();

        Self(positions)
    }
}

fn common_solve(input: &str, expension_factor: usize) -> usize {
    let galaxy_positions = GalaxyPositions::parse(input, expension_factor);

    let galaxy_pairs = (0..galaxy_positions.0.len()-1)
        .flat_map(|lhs_i| {
            let mut tmp = Vec::new();

            for rhs_i in lhs_i+1..galaxy_positions.0.len() {
                tmp.push((lhs_i, rhs_i));
            }

            tmp
        })
        .collect::<Vec<(usize, usize)>>();

    galaxy_pairs
        .iter()
        .map(|(lhs_i, rhs_i)| (
            galaxy_positions.0.get(*lhs_i).unwrap(),
            galaxy_positions.0.get(*rhs_i).unwrap(),
        ))
        .map(|(lhs, rhs)| {
            let x_diff = (lhs.0 as i32 - rhs.0 as i32).abs() as usize;
            let y_diff = (lhs.1 as i32 - rhs.1 as i32).abs() as usize;

            x_diff + y_diff
        })
        .sum()
}

#[test]
fn example_1() {
    let result = common_solve(include_str!("../res/example"), 2);
    assert_eq!(result, 374);
}

#[test]
fn example_2() {
    let result = common_solve(include_str!("../res/example"), 10);
    assert_eq!(result, 1030);
}

#[test]
fn example_3() {
    let result = common_solve(include_str!("../res/example"), 100);
    assert_eq!(result, 8410);
}

fn main() {
    let result = common_solve(include_str!("../res/input"), 1000000);
    println!("result={result}");
}
