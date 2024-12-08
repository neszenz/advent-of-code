#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn from(input: &str) -> Report {
        let levels : Vec<i32> = input
            .split(' ')
            .map(|v| v.parse::<i32>().unwrap())
            .collect()
        ;

        Report{ levels }
    }

    fn is_safe(self: &Self) -> bool {
        let changes : Vec<i32> = self.levels
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect()
        ;

        let all_positive = changes.iter().all(|c| c.is_positive());
        let all_negative = changes.iter().all(|c| c.is_negative());
        let all_moderate = changes.iter().all(|c| c.abs() >= 1 && c.abs() <= 3);

        (all_positive || all_negative) && all_moderate
    }

    fn is_safe_v2(self: &Self) -> bool {
        let any_simplified_report_safe = (0..self.levels.len())
            .map(|i| {
                let mut copy = self.clone();
                copy.levels.remove(i);

                copy.is_safe()
            })
            .any(|result| result)
        ;
        
        any_simplified_report_safe
    }
}

fn solve_part_1(input: &str) -> usize {
    let reports : Vec<Report> = input
        .lines()
        .map(|l| Report::from(l))
        .collect()
    ;

    let n_safe : usize = reports
        .iter()
        .map(|r| r.is_safe())
        .filter(|flag| *flag)
        .count()
    ;

    n_safe
}

fn solve_part_2(input: &str) -> usize {
    let reports : Vec<Report> = input
        .lines()
        .map(|l| Report::from(l))
        .collect()
    ;

    let n_safe : usize = reports
        .iter()
        .map(|r| r.is_safe_v2())
        .filter(|flag| *flag)
        .count()
    ;

    n_safe
}

#[test]
fn example_part_1() {
    let result = solve_part_1(include_str!("../res/example"));
    assert_eq!(result, 2);
}

#[test]
fn example_part_2() {
    let result = solve_part_2(include_str!("../res/example"));
    assert_eq!(result, 4);
}

fn main() {
    let result_part_1 = solve_part_1(include_str!("../res/input"));
    let result_part_2 = solve_part_2(include_str!("../res/input"));
    println!("result_part_1={} result_part_2={}", result_part_1, result_part_2);
}
