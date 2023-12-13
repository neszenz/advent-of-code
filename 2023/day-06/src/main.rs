#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn n_ways_to_win(self: &Self) -> usize {
        (0..=self.time)
            .map(|i| i * (self.time - i))
            .filter(|d| d > &self.distance)
            .count()
    }
}

#[derive(Clone, Debug)]
struct RaceTable (Vec<Race>);

impl From<&str> for RaceTable {
    fn from(input: &str) -> Self {
        assert_eq!(input.lines().count(), 2);
        assert!(input.lines().nth(0).unwrap().starts_with("Time:"));
        assert!(input.lines().nth(1).unwrap().starts_with("Distance:"));

        let parse_column = |i: usize| -> i64 {
            input
                .lines()
                .nth(i)
                .unwrap()
                .split(' ')
                .skip(1)
                .filter(|l| !l.is_empty())
                .map(|ele| ele.to_string())
                .reduce(|lhs,rhs| lhs + &rhs)
                .unwrap()
                .parse::<i64>()
                .unwrap()
        };

        let time: i64 = parse_column(0);
        let distance: i64 = parse_column(1);

        RaceTable (
            Vec::from(
                [
                    Race {
                        time,
                        distance,
                    }
                ]
            )
        )
    }
}

impl RaceTable {
    fn eval(self: &Self) -> usize {
        self.0
            .iter()
            .map(|race| race.n_ways_to_win())
            .product()
    }
}

#[test]
fn example() {
    static EXAMPLE_INPUT: &str = include_str!("../res/example");
    static EXAMPLE_ANSWER: usize = 71503;

    let race_table = RaceTable::from(EXAMPLE_INPUT);

    let result: usize = race_table.eval();
    assert_eq!(result, EXAMPLE_ANSWER);
}

fn main() {
    static INPUT: &str = include_str!("../res/input");

    let race_table = RaceTable::from(INPUT);

    let result: usize = race_table.eval();
    println!("result={result}");
}
