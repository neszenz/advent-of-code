use std::collections::HashSet;

const RESOURCE_FILE_PATH: &str = "res/input";

type SchematicData = Vec<Vec<char>>;

struct SchematicMetaData {
    n_rows: usize,
    n_columns: usize,
    symbols: HashSet<char>,
}

struct SchematicNumberPosition {
    i: usize,
    j_begin: usize,
    j_end: usize,
}

struct SchematicSymbolPosition {
    i: usize,
    j: usize,
}

struct SchematicNumberBoundingBox {
    i_min: usize,
    i_max: usize,
    j_min: usize,
    j_max: usize,
}

fn parse_engine_schematic(input: &String) -> (SchematicData, SchematicMetaData) {
    let schematic: SchematicData = input
        .lines()
        .map(|l| l
            .chars()
            .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    let n_rows = schematic.len();

    let n_columns = if n_rows == 0 { 0 } else { schematic[0].len() };
    if !schematic.iter().all(|row| row.len() == n_columns ) {
        panic!("inconsistent row length");
    }

    let symbols = {
        let mut tmp = schematic.iter().flatten().filter(|c| !c.is_numeric() && **c != '.').cloned().collect::<Vec<char>>();
        tmp.sort();
        tmp.dedup();

        HashSet::from_iter(tmp.iter().cloned())
    };

    (schematic, SchematicMetaData{ n_rows, n_columns, symbols })
}

fn scan_for_candidates(schematic: &SchematicData, meta: &SchematicMetaData) -> Vec<SchematicNumberPosition> {
    let mut found_numbers: Vec<SchematicNumberPosition> = Vec::new();
    let mut scan_buffer: Vec<usize> = Vec::new();

    for i in 0..meta.n_rows {
        for j in 0..meta.n_columns {
            let cell = schematic[i][j];

            if cell.is_numeric() {
                scan_buffer.push(j);
            }
            else if scan_buffer.len() > 0 {
                found_numbers.push(
                    SchematicNumberPosition{
                        i,
                        j_begin: *scan_buffer.first().unwrap(),
                        j_end: *scan_buffer.last().unwrap(),
                    }
                );
                scan_buffer.clear();
            }
        }

        if !scan_buffer.is_empty() {
            found_numbers.push(
                SchematicNumberPosition{
                    i,
                    j_begin: *scan_buffer.first().unwrap(),
                    j_end: *scan_buffer.last().unwrap(),
                }
            );
            scan_buffer.clear();
        }
    }

    found_numbers
}

fn snbb_from_number_position(meta: &SchematicMetaData, pos: &SchematicNumberPosition) -> SchematicNumberBoundingBox {
    SchematicNumberBoundingBox{
        i_min: if pos.i == 0 { pos.i } else { pos.i - 1 },
        i_max: if pos.i == meta.n_rows - 1 { pos.i } else { pos.i + 1 },
        j_min: if pos.j_begin == 0 { pos.j_begin } else { pos.j_begin - 1 },
        j_max: if pos.j_end == meta.n_columns - 1 { pos.j_end } else { pos.j_end + 1},
    }
}

fn is_part_number(schematic: &SchematicData, meta: &SchematicMetaData, candidate_pos: &SchematicNumberPosition) -> bool {
    let snbb = snbb_from_number_position(meta, candidate_pos);

    for i in snbb.i_min..=snbb.i_max {
        for j in snbb.j_min..=snbb.j_max {
            if meta.symbols.contains(&schematic[i][j]) {
                return true;
            }
        }
    }

    return false;
}

fn value_of_schematic_number(schematic: &SchematicData, pos: &SchematicNumberPosition) -> i32 {
    let mut number_string: String = String::new();

    for j in pos.j_begin..=pos.j_end {
        number_string.push(schematic[pos.i][j]);
    }

    number_string.parse::<i32>().unwrap()
}

fn main() {
    let input = std::fs::read_to_string(RESOURCE_FILE_PATH).expect("resource file can be loaded");

    let (schematic, meta) = parse_engine_schematic(&input);

    let part_number_candidates = scan_for_candidates(&schematic, &meta);
    let part_numbers = part_number_candidates.iter().filter(|c| is_part_number(&schematic, &meta, c)).collect::<Vec<&SchematicNumberPosition>>();

    let gears_candidates = {
        let mut tmp: Vec<SchematicSymbolPosition> = Vec::new();

        for i in 0..meta.n_rows {
            for j in 0..meta.n_columns {
                if schematic[i][j] == '*' {
                    tmp.push(SchematicSymbolPosition { i , j });
                }
            }
        }

        tmp
    };

    let result: i32 = gears_candidates
        .iter()
        .map(|g| {
            let adjacent_part_numbers = part_numbers
                .iter()
                .filter(|pn| {
                    let bb = snbb_from_number_position(&meta, pn);
                    bb.i_min <= g.i && g.i <= bb.i_max &&
                    bb.j_min <= g.j && g.j <= bb.j_max
                })
                .cloned()
                .collect::<Vec<&SchematicNumberPosition>>();

            if adjacent_part_numbers.len() == 2 {
                adjacent_part_numbers.iter().map(|pn| value_of_schematic_number(&schematic, pn)).product()
            }
            else {
                0
            }
        })
        .sum();

    println!("result={}", result);
}
