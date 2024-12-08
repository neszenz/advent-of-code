fn solve(input: &str) -> i32 {
    let pared_input : Vec<(i32, i32)> = input
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
        let mut list = pared_input
            .iter()
            .map(|pair| pair.0)
            .collect::<Vec<i32>>()
        ;
        
        list.sort();

        list
    };

    let list_1 : Vec<i32> = {
        let mut list = pared_input
            .iter()
            .map(|pair| pair.1)
            .collect::<Vec<i32>>()
        ;
        
        list.sort();

        list
    };

    assert_eq!(list_0.len(), list_1.len());

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

#[test]
fn example() {
    let result = solve(include_str!("../res/example"));
    assert_eq!(result, 11);
}

fn main() {
    let result = solve(include_str!("../res/input"));
    println!("result={}", result);
}
