#![allow(dead_code)]

use cartesian::cartesian;

const NORTH: (i32, i32) = (-1, 0);
const SOUTH: (i32, i32) = (1, 0);
const EAST: (i32, i32) = (0, 1);
const WEST: (i32, i32) = (0, -1);

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
}

impl Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Start,
            '.' => Tile::Ground,
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::NorthEastPipe,
            'J' => Tile::NorthWestPipe,
            '7' => Tile::SouthWestPipe,
            'F' => Tile::SouthEastPipe,
            unknown => panic!("Unknown tile char {}", unknown),
        }
    }

    fn opening_offsets(self: &Self) -> Option<Vec<(i32, i32)>> {
        match self {
            Tile::VerticalPipe => Some([NORTH, SOUTH].into()),
            Tile::HorizontalPipe => Some([EAST, WEST].into()),
            Tile::NorthEastPipe => Some([NORTH, EAST].into()),
            Tile::NorthWestPipe => Some([NORTH, WEST].into()),
            Tile::SouthWestPipe => Some([SOUTH, WEST].into()),
            Tile::SouthEastPipe => Some([SOUTH, EAST].into()),
            Tile::Start => Some([NORTH, SOUTH, EAST, WEST].into()),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct PipeMap {
    tiles: Vec<Vec<Tile>>,
}

impl PipeMap {
    fn parse(input: &str) -> Self {
        assert!(input.lines().all(|line| line.len() == input.lines().nth(0).unwrap().len()));

        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|c| Tile::from(c))
                    .collect()
            })
            .collect();

        Self{ tiles }
    }

    fn find_start(self: &Self) -> (usize, usize) {
        let start_positions = self.tiles
            .iter()
            .enumerate()
            .flat_map(|(x, line)| line
                .iter()
                .enumerate()
                .filter(|(_, tile)| **tile == Tile::Start)
                .map(|(y, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
            )
            .collect::<Vec<(usize, usize)>>();

        assert_eq!(start_positions.len(), 1);

        start_positions[0]
    }

    fn is_on_grid(self: &Self, pos: (usize, usize)) -> bool {
        pos.0 < self.tiles.len() && pos.1 < self.tiles[pos.0].len()
    }

    fn are_connected(self: &Self, pos_a: (usize, usize), pos_b: (usize, usize)) -> bool {
        if !self.is_on_grid(pos_a) || !self.is_on_grid(pos_b) {
            return false;
        }

        let a_to_b_offset: (i32, i32) = (
            pos_b.0 as i32 - pos_a.0 as i32,
            pos_b.1 as i32 - pos_a.1 as i32,
        );
        let b_to_a_offset: (i32, i32) = (
            pos_a.0 as i32 - pos_b.0 as i32,
            pos_a.1 as i32 - pos_b.1 as i32,
        );

        let tile_a = &self.tiles[pos_a.0][pos_a.1];
        let tile_b = &self.tiles[pos_b.0][pos_b.1];

        let a_open_to_b = match tile_a.opening_offsets() {
            Some(offsets) => offsets.contains(&a_to_b_offset),
            None => false,
        };
        let b_open_to_a = match tile_b.opening_offsets() {
            Some(offsets) => offsets.contains(&b_to_a_offset),
            None => false,
        };

        a_open_to_b && b_open_to_a
    }

    fn get_connected_neighborhood(self: &Self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let x_start = pos.0.min(0);
        let x_end = pos.0.max(self.tiles[0].len() - 1);

        let y_start = pos.1.min(0);
        let y_end = pos.1.max(self.tiles.len() - 1);

        cartesian!(x_start..=x_end, y_start..=y_end)
            .filter(|(x, y)| !(&pos.0 == x && &pos.1 == y) && self.are_connected(pos, (*x, *y)))
            .collect()
    }

    fn solve_part_1(self: &Self) -> i32 {
        let mut visited_tiles: Vec<(usize, usize)> = Vec::new();
        let mut todo_items: Vec<(usize, usize)> = [self.find_start()].into();
        let mut possible_solutions: Vec<i32> = Vec::new();

        let mut step_counter = 0;
        while !todo_items.is_empty() {
            let mut next_step_todo_items: Vec<(usize, usize)> = Vec::new();

            for pos in todo_items {
                if visited_tiles.contains(&pos) {
                    continue;
                }

                visited_tiles.push(pos);

                let neighborhood = self.get_connected_neighborhood(pos);

                if neighborhood.is_empty() { // dead end
                    println!("dead end");
                    break;
                }

                let mut unvisited_neighborhood: Vec<(usize, usize)> = neighborhood
                    .iter()
                    .filter(|neighbor_pos| !visited_tiles.contains(neighbor_pos))
                    .cloned()
                    .collect();

                if unvisited_neighborhood.is_empty() { // loop end
                    println!("loop end at {step_counter}");
                    possible_solutions.push(step_counter);
                }

                next_step_todo_items.append(&mut unvisited_neighborhood);
            }

            step_counter += 1;
            todo_items = next_step_todo_items;
        }

        assert!(!possible_solutions.is_empty());

        *possible_solutions.iter().max().unwrap()
    }
}

#[test]
fn pipe_map_is_on_grid() {
    let pipe_map = PipeMap::parse(include_str!("../res/example"));

    assert!(pipe_map.is_on_grid((0, 0)));
    assert!(pipe_map.is_on_grid((0, 4)));
    assert!(pipe_map.is_on_grid((4, 0)));
    assert!(pipe_map.is_on_grid((4, 4)));
    
    assert!(!pipe_map.is_on_grid((0, 5)));
    assert!(!pipe_map.is_on_grid((5, 3)));
}

#[test]
fn pipe_map_are_connected() {
    let pipe_map = PipeMap::parse(include_str!("../res/example"));

    assert!(pipe_map.are_connected((0, 2), (0, 3)));
    assert!(pipe_map.are_connected((0, 2), (1, 2)));
    assert!(pipe_map.are_connected((1, 1), (1, 2)));
    assert!(pipe_map.are_connected((1, 1), (2, 1)));
    assert!(pipe_map.are_connected((2, 0), (2, 1)));
    assert!(pipe_map.are_connected((2, 0), (3, 0)));
    assert!(pipe_map.are_connected((3, 3), (3, 4)));

    assert!(!pipe_map.are_connected((1, 2), (1, 3)));
    assert!(!pipe_map.are_connected((0, 0), (1, 0)));
    assert!(!pipe_map.are_connected((3, 0), (3, 1)));
    assert!(!pipe_map.are_connected((2, 0), (3, 1)));
    assert!(!pipe_map.are_connected((3, 3), (2, 3)));
    assert!(!pipe_map.are_connected((2, 0), (1, 0)));
}

#[test]
fn example_part_1() {
    static INPUT: &str = include_str!("../res/example");
    static ANSWER: i32 = 8;

    let pipe_map = PipeMap::parse(INPUT);
    let result: i32 = pipe_map.solve_part_1();
    assert_eq!(result, ANSWER);
}

fn main() {
    static INPUT: &str = include_str!("../res/input");

    let pipe_map = PipeMap::parse(INPUT);
    let result: i32 = pipe_map.solve_part_1();
    println!("result={result}");
}
