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

fn scan_for_candidats(schematic: &SchematicData, meta: &SchematicMetaData) -> Vec<SchematicNumberPosition> {
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

fn is_part_number(schematic: &SchematicData, meta: &SchematicMetaData, candidat_pos: &SchematicNumberPosition) -> bool {
    let i_begin = if candidat_pos.i == 0 { candidat_pos.i } else { candidat_pos.i - 1 };
    let i_end = if candidat_pos.i == meta.n_rows - 1 { candidat_pos.i } else { candidat_pos.i + 1 };
    let j_begin = if candidat_pos.j_begin == 0 { candidat_pos.j_begin } else { candidat_pos.j_begin - 1 };
    let j_end = if candidat_pos.j_end == meta.n_columns - 1 { candidat_pos.j_end } else { candidat_pos.j_end + 1};

    for i in i_begin..=i_end {
        for j in j_begin..=j_end {
            let cell = schematic[i][j];
            if meta.symbols.contains(&cell) {
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

    let candidats = scan_for_candidats(&schematic, &meta);
    let part_numbers = candidats.iter().filter(|c| is_part_number(&schematic, &meta, c)).collect::<Vec<&SchematicNumberPosition>>();

    let result: i32 = part_numbers.iter().map(|n| value_of_schematic_number(&schematic, n)).sum();
    println!("result={}", result);
}
