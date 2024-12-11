use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct OrderingRules(HashMap<i32, HashSet<i32>>);

impl OrderingRules {
    fn from(input: &str) -> Self {
        let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();

        for line in input.lines() {
            let split_line = line.split('|').collect::<Vec<&str>>();
            assert_eq!(split_line.len(), 2);

            let lhs = split_line.get(0).unwrap().parse::<i32>().unwrap();
            let rhs = split_line.get(1).unwrap().parse::<i32>().unwrap();

            rules_map.entry(lhs).or_insert([rhs].into()).insert(rhs);
        }

        Self(rules_map)
    }

    fn is_ordered_pair(self: &Self, lhs: i32, rhs: i32) -> bool {
        self.0.contains_key(&lhs) && self.0.get(&lhs).unwrap().contains(&rhs)
    }
}

#[derive(Debug)]
struct PageUpdates(Vec<Vec<i32>>);

impl PageUpdates {
    fn from(input: &str) -> Self {
        PageUpdates(
            input
                .lines()
                .map(|update_string| {
                    update_string
                        .split(',')
                        .map(|page_id_string| page_id_string.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        )
    }

    fn sum_middle_page_numbers(self: &Self) -> i32 {
        self.0
            .iter()
            .map(|page_update| {
                assert!(page_update.len() % 2 != 0);
                let middle_index = page_update.len() / 2;
                page_update.get(middle_index).unwrap()
            })
            .sum()
    }
}

fn solve(input: &str) -> i32 {
    let input_sections = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(input_sections.len(), 2);

    let ordering_rules = OrderingRules::from(input_sections.get(0).unwrap());
    let page_updates = PageUpdates::from(input_sections.get(1).unwrap());

    let correctly_ordered_updates = PageUpdates(
        page_updates.0
            .iter()
            .filter(|page_update| page_update
                .windows(2)
                .all(|ids| {
                    assert_eq!(ids.len(), 2);
                    let lhs = ids.get(0).unwrap();
                    let rhs = ids.get(1).unwrap();
                    ordering_rules.is_ordered_pair(*lhs, *rhs)
                })
            )
            .cloned()
            .collect::<Vec<Vec<i32>>>()
    );

    correctly_ordered_updates.sum_middle_page_numbers()
}

#[test]
fn example() {
    let result = solve(include_str!("../res/example"));
    assert_eq!(result, 143);
}

fn main() {
    let result = solve(include_str!("../res/input"));
    println!("result={result}");
}
