const RESOURCE_FILE_PATH: &str = "res/input";

struct AlmanacMapEntry {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64
}

impl ToString for AlmanacMapEntry {
    fn to_string(&self) -> String {
        format!("{{ {}, {}, {} }}", self.destination_range_start, self.source_range_start, self.range_length)
    }
}

struct AlmanacMap {
    entries: Vec<AlmanacMapEntry>,
}

impl From<&Vec<String>> for AlmanacMap {
    fn from(section: &Vec<String>) -> Self {
        assert!(section.len() > 2);

        let header = &section[0];
        assert!(header.ends_with("map:"));

        let entries = {
            section
                .iter()
                .skip(1)
                .map(|ele| {
                    let tmp = ele
                        .split(' ')
                        .map(|number_string| number_string.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();

                    assert!(tmp.len() == 3);

                    AlmanacMapEntry {
                        destination_range_start: tmp[0],
                        source_range_start: tmp[1],
                        range_length: tmp[2],
                    }
                })
                .collect()
        };

        AlmanacMap { entries }
    }
}

impl ToString for AlmanacMap {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result += format!("map (#entries={}):", self.entries.len()).as_str();

        for entry in &self.entries {
            result += format!("\n[{}, {}, {}]", entry.destination_range_start, entry.source_range_start, entry.range_length).as_str();
        }

        result
    }
}

impl AlmanacMap {
    fn resolve(self: &Self, source: i64) -> i64 {
        let corresponding_entry_option = {
            assert!(!self.entries.is_empty());

            self.entries
                .iter()
                .filter(|e| e.source_range_start <= source)
                .max_by(|lhs, rhs| lhs.source_range_start.cmp(&rhs.source_range_start))
        };

        if corresponding_entry_option.is_none() {
            return source;
        }

        let corresponding_entry = corresponding_entry_option.unwrap();

        assert!(corresponding_entry.source_range_start <= source);
        let range_offset = source - corresponding_entry.source_range_start;

        if range_offset <= corresponding_entry.range_length {
            return corresponding_entry.destination_range_start + range_offset;
        }
        else {
            return source;
        }
    }
}

struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil_map: AlmanacMap,
    soil_to_fertilizer_map: AlmanacMap,
    fertilizer_to_water_map: AlmanacMap,
    water_to_light_map: AlmanacMap,
    light_to_temperature_map: AlmanacMap,
    temperature_to_humidity_map: AlmanacMap,
    humidity_to_location_map: AlmanacMap,
}

impl From<&String> for Almanac {
    fn from(input: &String) -> Self {
        let sections: Vec<Vec<String>> = input
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .split(|l| l.is_empty())
            .map(|l| l.to_vec())
            .collect();

        assert!(sections.len() == 8);

        let seeds: Vec<i64> = {
            let content = &sections[0];
            assert!(content.len() == 1);

            let header_and_seed_list = content[0].split(' ').map(|ele| ele.to_string()).collect::<Vec<String>>();
            assert!(header_and_seed_list[0] == "seeds:");

            header_and_seed_list
                .iter()
                .skip(1)
                .map(|ele| ele.parse::<i64>().unwrap() )
                .collect::<Vec<i64>>()
        };

        let seed_to_soil_map = AlmanacMap::from(&sections[1]);
        let soil_to_fertilizer_map = AlmanacMap::from(&sections[2]);
        let fertilizer_to_water_map = AlmanacMap::from(&sections[3]);
        let water_to_light_map = AlmanacMap::from(&sections[4]);
        let light_to_temperature_map = AlmanacMap::from(&sections[5]);
        let temperature_to_humidity_map = AlmanacMap::from(&sections[6]);
        let humidity_to_location_map = AlmanacMap::from(&sections[7]);

        Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(RESOURCE_FILE_PATH).expect("resource file can be loaded");

    let almanac = Almanac::from(&input);

    let result: i64 = almanac.seeds
        .iter()
        .map(|seed| {
            let soil = almanac.seed_to_soil_map.resolve(*seed);
            let fertilizer = almanac.soil_to_fertilizer_map.resolve(soil);
            let water = almanac.fertilizer_to_water_map.resolve(fertilizer);
            let light = almanac.water_to_light_map.resolve(water);
            let temperature = almanac.light_to_temperature_map.resolve(light);
            let humidity = almanac.temperature_to_humidity_map.resolve(temperature);
            let location = almanac.humidity_to_location_map.resolve(humidity);

            location
        })
        .min()
        .unwrap();

    println!("result={}", result);
}
