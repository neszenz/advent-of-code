#![allow(dead_code)]

#[derive(Debug)]
struct History (Vec<i64>);

impl History {
    fn predict_next(self: &Self) -> i64 {
        fn compute_derivative(values: &Vec<i64>) -> Vec<i64> {
            values
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect()
        }

        let mut derivatives: Vec<Vec<i64>> = Vec::from([compute_derivative(&self.0)]);

        while derivatives.last().unwrap().iter().any(|ele| *ele != 0) {
            derivatives.push(compute_derivative(derivatives.last().unwrap()));
        }

        let prediction = self.0.last().unwrap() + derivatives.iter().map(|d| d.last().unwrap()).sum::<i64>();

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
fn example() {
    static INPUT: &str = include_str!("../res/example");
    static ANSWER: i64 = 114;

    let oasis = Oasis::parse(INPUT);

    let result: i64 = oasis.0
        .iter()
        .map(|h| h.predict_next())
        .sum();

    assert_eq!(result, ANSWER);
}

fn main() {
    static INPUT: &str = include_str!("../res/input");

    let oasis = Oasis::parse(INPUT);

    let result: i64 = oasis.0
        .iter()
        .map(|h| h.predict_next())
        .sum();
    println!("result={result}");
}
