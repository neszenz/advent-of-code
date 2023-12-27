#![allow(dead_code)]

use cartesian::cartesian;

const NORTH: (i32, i32) = (-1, 0);
const SOUTH: (i32, i32) = (1, 0);
const EAST: (i32, i32) = (0, 1);
const WEST: (i32, i32) = (0, -1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NorthSouthPipe,
    EastWestPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    GroundUndecided,
    GroundClockwise,
    GroundCounterClockwise,
    Start,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::NorthSouthPipe,
            '-' => Self::EastWestPipe,
            'L' => Self::NorthEastPipe,
            'J' => Self::NorthWestPipe,
            '7' => Self::SouthWestPipe,
            'F' => Self::SouthEastPipe,
            '.' => Self::GroundUndecided,
            'S' => Self::Start,
            unknown => panic!("unknown tile char {unknown}"),
        }
    }

    fn to_unicode(self: &Self) -> char {
        match self {
            Tile::NorthSouthPipe => '│',
            Tile::EastWestPipe => '─',
            Tile::NorthEastPipe => '└',
            Tile::NorthWestPipe => '┘',
            Tile::SouthWestPipe => '┐',
            Tile::SouthEastPipe => '┌',
            Tile::GroundUndecided => '·',
            Tile::GroundClockwise => 'C',
            Tile::GroundCounterClockwise => 'Ɔ',
            Tile::Start => '┼',
        }
    }

    fn openings(self: &Self) -> Vec<(i32, i32)> {
        match self {
            Tile::NorthSouthPipe => [NORTH, SOUTH].into(),
            Tile::EastWestPipe => [EAST, WEST].into(),
            Tile::NorthEastPipe => [NORTH, EAST].into(),
            Tile::NorthWestPipe => [NORTH, WEST].into(),
            Tile::SouthWestPipe => [SOUTH, WEST].into(),
            Tile::SouthEastPipe => [SOUTH, EAST].into(),
            Tile::GroundUndecided => [].into(),
            Tile::GroundClockwise => [].into(),
            Tile::GroundCounterClockwise => [].into(),
            Tile::Start => [NORTH, SOUTH, EAST, WEST].into(),
        }
    }
}

#[derive(Debug)]
struct TileMap {
    data: Vec<Vec<Tile>>,
    len_x: i32,
    len_y: i32,
}

impl TileMap {
    fn parse(input: &str) -> Self {
        let data: Vec<Vec<Tile>> = input
            .lines()
            .map(|l| l
                .chars()
                .map(|c| Tile::from(c))
                .collect()
            )
            .collect();

        assert!(!data.is_empty());

        let len_x = data.len() as i32;
        let len_y = data[0].len() as i32;

        assert!(data.iter().all(|l| l.len() as i32 == len_y));

        Self { data, len_x, len_y }
    }

    fn is_on_map(self: &Self, pos: (i32, i32)) -> bool {
        0 <= pos.0 && pos.0 < self.len_x && 0 <= pos.1 && pos.1 < self.len_y
    }

    fn are_connected(self: &Self, a: (i32, i32), b: (i32, i32)) -> bool {
        let a_to_b_offset = (
            b.0 - a.0,
            b.1 - a.1,
        );
        let b_to_a_offset = (
            a.0 - b.0,
            a.1 - b.1,
        );

        let a_open_to_b = self.at(a)
            .unwrap()
            .openings()
            .contains(&a_to_b_offset);

        let b_open_to_a = self.at(b)
            .unwrap()
            .openings()
            .contains(&b_to_a_offset);

        a_open_to_b && b_open_to_a
    }

    fn at(self: &Self, pos: (i32, i32)) -> Option<Tile> {
        if self.is_on_map(pos) {
            Some(self.data[pos.0 as usize][pos.1 as usize])
        }
        else {
            None
        }
    }

    fn move_on_grid(self: &Self, pos: (i32, i32), offset: (i32, i32)) -> Option<(i32, i32)> {
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
        
        if self.is_on_map(new_pos) {
            Some(new_pos)
        }
        else {
            None
        }
    }

    fn start(self: &Self) -> (i32, i32) {
        let start_tiles: Vec<(i32,i32)> = self.data
            .iter()
            .enumerate()
            .flat_map(|(x, line)| line
                .iter()
                .enumerate()
                .filter(|(_, tile)| **tile == Tile::Start)
                .map(|(y, _)| (x as i32,y as i32))
                .collect::<Vec<(i32,i32)>>()
            )
            .collect();

        assert_eq!(start_tiles.len(), 1);

        start_tiles[0]
    }

    fn connected_neighbors_of(self: &Self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        match self.at(pos) {
            Some(tile) => tile
                .openings()
                .iter()
                .map(|offset| self.move_on_grid(pos, *offset))
                .filter(|ele| ele.is_some())
                .map(|ele| ele.unwrap())
                .filter(|new_pos| self.are_connected(pos, *new_pos))
                .collect::<Vec<(i32, i32)>>(),

            None => Vec::new(),
        }
    }

    fn trace_loop(self: &Self) -> Vec<(i32, i32)> {
        let start = self.start();

        let connected_neighbors_of_start = self.connected_neighbors_of(start);
        assert_eq!(connected_neighbors_of_start.len(), 2);

        let mut path: Vec<(i32, i32)> = [start].into();
        let mut curr_pos = connected_neighbors_of_start[0];

        loop {
            path.push(curr_pos);

            let curr_connected_neighbors: Vec<(i32, i32)> = self
                .connected_neighbors_of(curr_pos)
                .iter()
                .filter(|pos| !path.contains(pos))
                .cloned()
                .collect();

            if curr_connected_neighbors.is_empty() {
                break;
            }

            assert_eq!(curr_connected_neighbors.len(), 1);
            let next_pos = curr_connected_neighbors[0];
            curr_pos = next_pos;
        }

        path
    }
}

fn solve_part_1(input: &str) -> i32 {
    let tile_map = TileMap::parse(input);
    let pipe_loop = tile_map.trace_loop();

    assert!(pipe_loop.len() % 2 == 0);
    (pipe_loop.len() / 2) as i32
}

fn surrounding_clockwise_of((x, y): (i32, i32), x_max: i32, y_max: i32) -> Vec<(i32, i32)> {
    let x_min = 0.max(x - 1);
    let y_min = 0.max(y - 1);

    [
        (x - 1, y), // top
        (x - 1, y + 1), // top-right
        (x, y + 1), // right
        (x + 1, y + 1), // bottom-right
        (x + 1, y), // bottom
        (x + 1, y - 1), // bottom-left
        (x, y - 1), // left
        (x - 1, y - 1), // top-left
    ]
        .iter()
        .filter(|ele| x_min <= ele.0 && ele.0 <= x_max && y_min <= ele.1 && ele.1 <= y_max)
        .cloned()
        .collect::<Vec<(i32, i32)>>()
}

fn solve_part_2(input: &str) -> usize {
    let mut tile_map = TileMap::parse(input);
    let pipe_loop = tile_map.trace_loop();

    let collect_region_around = |pos: (i32, i32)| -> Vec<(i32, i32)> {
        let mut collected: Vec<(i32, i32)> = [].into();
        let mut horizon: Vec<(i32, i32)> = [pos].into();

        while !horizon.is_empty() {
            let curr_pos = horizon.pop().unwrap();

            if !pipe_loop.contains(&curr_pos) && !collected.contains(&curr_pos) {
                horizon.append(&mut surrounding_clockwise_of(curr_pos, tile_map.len_x - 1, tile_map.len_y - 1));
                collected.push(curr_pos);
            }
        }

        collected
    };

    let search_space: Vec<(i32, i32)> = cartesian!(0..tile_map.len_x, 0..tile_map.len_y).collect();
    for curr_pos in search_space {
        if pipe_loop.contains(&curr_pos) || [Tile::GroundClockwise, Tile::GroundCounterClockwise].contains(&tile_map.at(curr_pos).unwrap()) {
            continue;
        }

        let mut loop_indices_in_clockwise_surrounding: Vec<usize> = surrounding_clockwise_of(curr_pos, tile_map.len_x - 1, tile_map.len_y - 1)
            .iter()
            .map(|pos|
                match pipe_loop.iter().position(|ele| *ele == *pos) {
                    Some(i) => Some(i),
                    None => None,
                }
            )
            .filter(|ele| ele.is_some())
            .map(|ele| ele.unwrap())
            .collect();

        if loop_indices_in_clockwise_surrounding.len() <= 2 {
            continue;
        }

        let is_clockwise_aligned = loop {
            let all_accenting = loop_indices_in_clockwise_surrounding
                .windows(2)
                .all(|w| w[0] < w[1]);

            let all_deccenting = loop_indices_in_clockwise_surrounding
                .windows(2)
                .all(|w| w[0] > w[1]);

            if !all_accenting && !all_deccenting {
                loop_indices_in_clockwise_surrounding.rotate_left(1);
                continue;
            }

            break all_accenting;
        };

        let ground_type = if is_clockwise_aligned {
            Tile::GroundClockwise
        }
        else {
            Tile::GroundCounterClockwise
        };

        collect_region_around(curr_pos)
            .iter()
            .for_each(|p| tile_map.data[p.0 as usize][p.1 as usize] = ground_type);
    }

    tile_map.data.iter().for_each(|line| println!("{}", line.iter().map(|tile| tile.to_unicode()).collect::<String>()));

    let inside_tile_type = {
        match tile_map.at((0, 0)) {
            Some(tile) => match tile {
                Tile::GroundClockwise => Tile::GroundCounterClockwise,
                Tile::GroundCounterClockwise => Tile::GroundClockwise,
                surprise => panic!("tile at (0,0) is {surprise:?}, but should only be decided ground"),
            },
            None => panic!("tile_map at (0,0) should not be empty"),
        }
    };

    tile_map.data
        .iter()
        .map(|line| line
            .iter()
            .filter(|tile| **tile == inside_tile_type)
            .count()
        )
        .sum::<usize>()
}

#[test]
fn example_1() {
    let result = solve_part_1(include_str!("../res/example_1"));
    assert_eq!(result, 8);
}

#[test]
fn example_2() {
    let result = solve_part_2(include_str!("../res/example_2"));
    assert_eq!(result, 4)
}

#[test]
fn example_3() {
    let result = solve_part_2(include_str!("../res/example_3"));
    assert_eq!(result, 8)
}

#[test]
fn example_4() {
    let result = solve_part_2(include_str!("../res/example_4"));
    assert_eq!(result, 10)
}

fn main() {
    let result = solve_part_2(include_str!("../res/input"));
    println!("result={result}");
}
