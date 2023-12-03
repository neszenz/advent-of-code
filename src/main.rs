const RESOURCE_FILE_PATH: &str = "res/input";

fn main() {
    let input = std::fs::read_to_string(RESOURCE_FILE_PATH).expect("resource file can be loaded");

    let result: i32 = input
        .lines()
        .map(|line| {
            line
                .chars()
                .filter(|char| char.is_numeric())
                .collect::<String>()
        })
        .map(|filtered| {
            let fd: char = filtered.chars().next().unwrap();
            let ld: char = filtered.chars().last().unwrap();
            (fd.to_string() + &ld.to_string()).parse::<i32>().unwrap()
        })
        .sum();

    println!("result={}", result);
}
