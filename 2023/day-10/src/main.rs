#![allow(dead_code)]

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
    Ground,
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
            '.' => Self::Ground,
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
            Tile::Ground => '·',
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
            Tile::Ground => [].into(),
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
}

fn solve_part_1(input: &str) -> i32 {
    let tile_map = TileMap::parse(input);

    let start = tile_map.start();

    let connected_neighbors_of_start = tile_map.connected_neighbors_of(start);
    assert_eq!(connected_neighbors_of_start.len(), 2);

    let mut path: Vec<(i32, i32)> = [start].into();
    let mut curr_pos = connected_neighbors_of_start[0];

    loop {
        path.push(curr_pos);

        let curr_connected_neighbors: Vec<(i32, i32)> = tile_map
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

    (path.len() / 2) as i32
}

#[test]
fn example_1() {
    let result: i32 = solve_part_1(include_str!("../res/example_1"));
    assert_eq!(result, 8);
}

fn main() {
    let result = solve_part_1(include_str!("../res/input"));
    println!("result={result}");
}
