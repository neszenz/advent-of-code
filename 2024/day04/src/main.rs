//  S     S     S
//    A   A   A  
//      M M M    
//  S A M X M A S
//      M M M    
//    A   A   A  
//  S     S     S

use std::collections::HashSet;

#[derive(Debug)]
struct WordSearch(Vec<Vec<char>>);

impl WordSearch {
    fn parse(input: &str) -> Self {
        let new_word_search = WordSearch(
            input
                .lines()
                .map(|l| {
                    l.chars().collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>()
        );

        // Make sure all row have same length
        assert_eq!(new_word_search.0
            .iter()
            .map(|row| row.len())
            .collect::<HashSet<usize>>()
            .len(),
            1
        );

        new_word_search
    }

    fn get(self: &Self, row_i: i32, column_j: i32) -> Option<char> {
        if row_i < 0 || column_j < 0 {
            return None;
        }

        let parsed_row_i = row_i as usize;
        if parsed_row_i >= self.0.len() {
            return None;
        }

        let row = self.0.get(parsed_row_i).unwrap();
        let parsed_column_j = column_j as usize;
        if parsed_column_j >= row.len() {
            return None;
        }

        return Some(*row.get(parsed_column_j).unwrap());
    }

    fn n_rows(self: &Self) -> i32 {
        self.0.len() as i32
    }

    fn n_columns(self: &Self) -> i32 {
        if self.0.is_empty() {
            0
        }
        else {
            self.0.get(0).unwrap().len() as i32
        }
    }

    fn trace(self: &Self, start_i: i32, start_j: i32, step_i: i32, step_j: i32) -> bool {
        let mut i = start_i;
        let mut j = start_j;

        for expected_c in ['X', 'M', 'A', 'S'] {
            let Some(actual_c) = self.get(i, j) else { return false; };

            if actual_c != expected_c {
                return false;
            }

            i += step_i;
            j += step_j;
        }

        return true;
    }
}

fn solve(input: &str) -> usize {
    let word_search = WordSearch::parse(input);

    let mut n_xmas_occurrences = 0;

    for i in 0..word_search.n_rows() {
        for j in 0..word_search.n_columns() {
            let n_traces = [
                (-1, -1),
                (-1,  0),
                (-1,  1),
                ( 0, -1),
                ( 0,  1),
                ( 1, -1),
                ( 1,  0),
                ( 1,  1),
            ].iter()
                .filter(|ij| word_search.trace(i, j, ij.0, ij.1))
                .count()
            ;
            n_xmas_occurrences += n_traces;
        }
    }

    n_xmas_occurrences
}

#[test]
fn example() {
    let result = solve(include_str!("../res/example"));
    assert_eq!(result, 18);
}

fn main() {
    let result = solve(include_str!("../res/input"));
    println!("result={result}");
}
