use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let parsed_input : Vec<(i32, i32)> = input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|v| {
            assert_eq!(v.len(), 4);
            assert_ne!(v[0], "");
            assert_eq!(v[1], "");
            assert_eq!(v[2], "");
            assert_ne!(v[3], "");

            (v.get(0).unwrap().parse::<i32>().unwrap(), v.get(3).unwrap().parse::<i32>().unwrap())
        })
        .collect()
    ;

    let list_0 : Vec<i32> = {
        let mut list = parsed_input
            .iter()
            .map(|pair| pair.0)
            .collect::<Vec<i32>>()
        ;
        
        list.sort();

        list
    };

    let list_1 : Vec<i32> = {
        let mut list = parsed_input
            .iter()
            .map(|pair| pair.1)
            .collect::<Vec<i32>>()
        ;
        
        list.sort();

        list
    };

    assert_eq!(list_0.len(), list_1.len());

    (list_0, list_1)
}

fn solve_part_1(input: &str) -> i32 {
    let (list_0, list_1) = parse_input(input);

    let distances : Vec<i32> = (0..list_0.len())
        .map(|i| (list_0.get(i).unwrap() - list_1.get(i).unwrap()).abs())
        .collect()
    ;

    let total_distance : i32 = distances
        .iter()
        .sum()
    ;

    return total_distance;
}

fn solve_part_2(input: &str) -> i32 {
    let (list_0, list_1) = parse_input(input);

    fn count_id_occurrences(list: &Vec<i32>) -> HashMap<i32, i32> {
        let mut map = HashMap::new();

        for value in list {
            map
                .entry(*value)
                .and_modify(|v| *v += 1)
                .or_insert(1)
            ;
        }

        map
    }

    let map_1 : HashMap<i32, i32> = count_id_occurrences(&list_1);

    let result : i32 = list_0
        .iter()
        .map(|v| {
            v * map_1.get(v).unwrap_or(&0)
        })
        .sum()
    ;

    result
}

#[test]
fn example_part_1() {
    let result = solve_part_1(include_str!("../res/example"));
    assert_eq!(result, 11);
}

#[test]
fn example_part_2() {
    let result = solve_part_2(include_str!("../res/example"));
    assert_eq!(result, 31);
}

fn main() {
    let result_part_1 = solve_part_1(include_str!("../res/input"));
    let result_part_2 = solve_part_2(include_str!("../res/input"));
    println!("result_part_1={} result_part_2={}", result_part_1, result_part_2);
}
