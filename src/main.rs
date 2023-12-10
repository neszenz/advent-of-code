const RESOURCE_FILE_PATH: &str = "res/input";

struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn to_string(&self) -> String {
        format!("{{start={}, length={}}}", self.start, self.length)
    }

    fn end(&self) -> i64 {
        if self.length == 0 {
            self.start
        }
        else { 
            self.start + self.length - 1
        }
    }
}

struct Mapping {
    transformation_tabel: Vec<(i64, i64)>,
}

impl Mapping {
    fn to_string(&self) -> String {
        let mut result: String = String::new();

        result += &format!("Mapping (#entries={}):\n", self.transformation_tabel.len());
        for e in &self.transformation_tabel {
            result += &format!("{}: {}\n", e.0, e.1);
        }

        result
    }

    fn apply_value(&self, value: i64) -> i64 {
        let offset = self.transformation_tabel
            .iter()
            .filter(|ele| ele.0 <= value)
            .last()
            .unwrap()
            .1;

        value + offset
    }

    fn apply_range(&self, range: &Range) -> Vec<Range> {
        let relevant_transformations: Vec<(i64,i64)> = {
            self.transformation_tabel
                .iter()
                .filter(|ele| range.start < ele.0)
                .cloned()
                .collect()
        };

        let mut result: Vec<Range> = Vec::new();

        let mut next_start_at = range.start;
        let mut remaining_length: i64 = range.length;

        for i in 0..relevant_transformations.len() {
            let t = &relevant_transformations[i];
            assert!(next_start_at < t.0);

            let length = if i+1 == relevant_transformations.len() {
                remaining_length
            }
            else {
                (t.0 - next_start_at).min(remaining_length)
            };

            result.push(
                Range {
                    start: self.apply_value(next_start_at),
                    length,
                }
            );

            next_start_at += length;
            remaining_length -= length;

            if remaining_length == 0 {
                break;
            }
        }

        if remaining_length > 0 {
            result.push(
                Range {
                    start: next_start_at,
                    length: remaining_length,
                }
            );
        }

        result
    }

    fn apply_ranges(&self, ranges: &Vec<Range>) -> Vec<Range> {
        ranges
            .iter()
            .flat_map(|ele| self.apply_range(ele))
            .collect()
    }
}


struct Almanac {
    seed_ranges: Vec<Range>,
    seed_to_soil_map: Mapping,
    soil_to_fertilizer_map: Mapping,
    fertilizer_to_water_map: Mapping,
    water_to_light_map: Mapping,
    light_to_temperature_map: Mapping,
    temperature_to_humidity_map: Mapping,
    humidity_to_location_map: Mapping,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let sections: Vec<Vec<&str>> = input
            .lines()
            .map(|l| l)
            .collect::<Vec<&str>>()
            .split(|l| l.is_empty())
            .map(|l| l.to_vec())
            .collect();

        assert!(sections.len() == 8);

        fn parse_seeds(input: &Vec<&str>) -> Vec<Range> {
            assert!(input.len() == 1);
            assert!(input[0].starts_with("seeds:"));

            input[0]
                .split(' ')
                .skip(1)
                .map(|ele| ele.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .chunks(2)
                .map(|chunk| Range { start: chunk[0], length: chunk[1] })
                .collect()
        }

        fn parse_mapping(input: &Vec<&str>) -> Mapping {
            assert!(input.len() >= 1);
            assert!(input[0].ends_with("map:"));

            struct MappingEntry {
                range: Range,
                offset: i64,
            }

            let mut mapping_entries: Vec<MappingEntry> = input
                .iter()
                .skip(1)
                .map(|ele| {
                    let tmp: Vec<i64> = ele
                        .split(' ')
                        .map(|number_string| number_string.parse::<i64>().unwrap())
                        .collect();

                    assert!(tmp.len() == 3);

                    let destination_range_start = tmp[0];
                    let source_range_start = tmp[1];
                    let range_length = tmp[2];

                    let offset = destination_range_start - source_range_start;

                    MappingEntry {
                        range: Range {
                            start: source_range_start,
                            length: range_length
                        },
                        offset
                    }
                })
                .collect();

            mapping_entries.sort_by(|lhs, rhs| lhs.range.start.cmp(&rhs.range.start));

            let transformation_tabel: Vec<(i64, i64)> = {
                let mut result: Vec<(i64,i64)> = Vec::new();

                let mut next_to_handle: i64 = 0;

                for entry in mapping_entries {
                    assert!(next_to_handle <= entry.range.start);

                    if next_to_handle < entry.range.start {
                        result.push((next_to_handle, 0));
                    }

                    result.push((entry.range.start, entry.offset));

                    next_to_handle = entry.range.end() + 1;
                }

                result.push((next_to_handle, 0));

                result
            };

            Mapping { transformation_tabel }
        }

        Almanac {
            seed_ranges: parse_seeds(&sections[0]),
            seed_to_soil_map: parse_mapping(&sections[1]),
            soil_to_fertilizer_map: parse_mapping(&sections[2]),
            fertilizer_to_water_map: parse_mapping(&sections[3]),
            water_to_light_map: parse_mapping(&sections[4]),
            light_to_temperature_map: parse_mapping(&sections[5]),
            temperature_to_humidity_map: parse_mapping(&sections[6]),
            humidity_to_location_map: parse_mapping(&sections[7]),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(RESOURCE_FILE_PATH).expect("resource file can be loaded");

    let almanac = Almanac::from(input.as_str());

    let soil_ranges = almanac.seed_to_soil_map.apply_ranges(&almanac.seed_ranges);
    let fertilizer_ranges = almanac.soil_to_fertilizer_map.apply_ranges(&soil_ranges);
    let water_ranges = almanac.fertilizer_to_water_map.apply_ranges(&fertilizer_ranges);
    let light_ranges = almanac.water_to_light_map.apply_ranges(&water_ranges);
    let temperature_ranges = almanac.light_to_temperature_map.apply_ranges(&light_ranges);
    let humidity_ranges = almanac.temperature_to_humidity_map.apply_ranges(&temperature_ranges);
    let location_ranges = almanac.humidity_to_location_map.apply_ranges(&humidity_ranges);

    let result = location_ranges
        .iter()
        .map(|ele| ele.start)
        .min()
        .unwrap();

    println!("result={}", result);
}
