#![allow(dead_code)]

fn compute_derivative(values: &Vec<i64>) -> Vec<i64> {
    values
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect()
}

fn compute_all_derivatives(values: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut derivatives: Vec<Vec<i64>> = Vec::from([compute_derivative(values)]);

    while derivatives.last().unwrap().iter().any(|ele| *ele != 0) {
        derivatives.push(compute_derivative(derivatives.last().unwrap()));
    }

    derivatives
}

#[derive(Debug)]
struct History (Vec<i64>);

impl History {
    fn predict_next(self: &Self) -> i64 {
        let derivatives = compute_all_derivatives(&self.0);
        let prediction = self.0.last().unwrap() + derivatives.iter().map(|d| d.last().unwrap()).sum::<i64>();

        prediction
    }

    fn extrapolate_backwards(self: &Self) -> i64 {
        let derivatives = compute_all_derivatives(&self.0);

        let mut val = 0;
        (0..derivatives.len())
            .rev()
            .for_each(|i| {
                if i + 1 == derivatives.len() {
                    val = *derivatives[i].first().unwrap();
                }
                else {
                    val = derivatives[i].first().unwrap() - val;
                }
            });

        let prediction = self.0.first().unwrap() - val;

        prediction
    }
}

#[derive(Debug)]
struct Oasis (Vec<History>);

impl Oasis {
    fn parse(input: &str) -> Self {
        let histories = input
            .lines()
            .map(|l| {
                History(
                    l
                        .split(' ')
                        .map(|number| number.parse().unwrap())
                        .collect()
                )
            })
            .collect();

        Oasis(histories)
    }
}

#[test]
fn example_part_1() {
    static INPUT: &str = include_str!("../res/example");
    static ANSWER: i64 = 114;

    let oasis = Oasis::parse(INPUT);

    let result: i64 = oasis.0
        .iter()
        .map(|h| h.predict_next())
        .sum();

    assert_eq!(result, ANSWER);
}

#[test]
fn example_part_2() {
    static INPUT: &str = include_str!("../res/example");
    static ANSWER: i64 = 2;

    let oasis = Oasis::parse(INPUT);

    let result: i64 = oasis.0
        .iter()
        .map(|h| h.extrapolate_backwards())
        .sum();

    assert_eq!(result, ANSWER);
}

fn main() {
    static INPUT: &str = include_str!("../res/input");

    let oasis = Oasis::parse(INPUT);

    let result: i64 = oasis.0
        .iter()
        .map(|h| h.extrapolate_backwards())
        .sum();
    println!("result={result}");
}
