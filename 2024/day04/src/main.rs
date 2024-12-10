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

    fn is_char_at(self: &Self, expected_c: char, row_i: i32, column_j: i32) -> bool {
        if row_i < 0 || column_j < 0 {
            return false;
        }

        let parsed_row_i = row_i as usize;
        if parsed_row_i >= self.0.len() {
            return false;
        }

        let row = self.0.get(parsed_row_i).unwrap();
        let parsed_column_j = column_j as usize;
        if parsed_column_j >= row.len() {
            return false;
        }

        *row.get(parsed_column_j).unwrap() == expected_c
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

    fn trace_xmas(self: &Self, start_i: i32, start_j: i32, step_i: i32, step_j: i32) -> bool {
        let mut i = start_i;
        let mut j = start_j;

        for expected_c in ['X', 'M', 'A', 'S'] {
            if !self.is_char_at(expected_c, i, j) {
                return false;
            }

            i += step_i;
            j += step_j;
        }

        return true;
    }

    fn trace_x_mas(self: &Self, start_i: i32, start_j: i32) -> bool {
        let center_fits = self.is_char_at('A', start_i, start_j);

        // variations
        //////////////////////////////////////
        // M   M // M   S // S   S // S   M //
        //   A   //   A   //   A   //   A   //
        // S   S // M   S // M   M // S   M //
        //////////////////////////////////////
        let any_variation_fits = [
            [('M', -1, -1), ('M', -1, 1), ('S', 1, -1), ('S', 1, 1)],
            [('M', -1, -1), ('S', -1, 1), ('M', 1, -1), ('S', 1, 1)],
            [('S', -1, -1), ('S', -1, 1), ('M', 1, -1), ('M', 1, 1)],
            [('S', -1, -1), ('M', -1, 1), ('S', 1, -1), ('M', 1, 1)],
        ]
            .iter()
            .any(|variations| variations
                .iter()
                .all(|expected_c_with_coordinates| {
                    let (c, i, j) = expected_c_with_coordinates;
                    self.is_char_at(*c, start_i + *i, start_j + *j)
                })
            )
        ;

        center_fits && any_variation_fits
    }
}

fn solve_part_1(input: &str) -> usize {
    let word_search = WordSearch::parse(input);

    let mut n_xmas_occurrences = 0;

    for i in 0..word_search.n_rows() {
        for j in 0..word_search.n_columns() {

            // traces
            ////////////////////
            //  S     S     S //
            //    A   A   A   //
            //      M M M     //
            //  S A M X M A S //
            //      M M M     //
            //    A   A   A   //
            //  S     S     S //
            ////////////////////
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
                .filter(|ij| word_search.trace_xmas(i, j, ij.0, ij.1))
                .count()
            ;
            n_xmas_occurrences += n_traces;
        }
    }

    n_xmas_occurrences
}

fn solve_part_2(input: &str) -> usize {
    let word_search = WordSearch::parse(input);

    let mut n_x_mas_occurrences = 0;

    for i in 0..word_search.n_rows() {
        for j in 0..word_search.n_columns() {
            if word_search.trace_x_mas(i, j) {
                n_x_mas_occurrences += 1;
            }
        }
    }

    n_x_mas_occurrences
}

#[test]
fn example_part_1() {
    let result = solve_part_1(include_str!("../res/example"));
    assert_eq!(result, 18);
}

#[test]
fn example_part_2() {
    let result = solve_part_2(include_str!("../res/example"));
    assert_eq!(result, 9);
}

fn main() {
    let result_part_1 = solve_part_1(include_str!("../res/input"));
    let result_part_2 = solve_part_2(include_str!("../res/input"));
    println!("result_part_1={result_part_1} result_part_2={result_part_2}");
}
