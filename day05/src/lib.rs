use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_input_into_raw_data(path: &str) -> HashMap<String, Vec<Vec<usize>>> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    let mut section = String::new();
    let mut map: HashMap<String, Vec<Vec<usize>>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        let mut line_contents = line.clone();

        if line.contains(':') {
            if line.ends_with(':') {
                section = line[0..line.len() - 1].to_string();
                line_contents.clear();
            } else {
                let line_split = line.split(": ").collect::<Vec<&str>>();
                section = line_split[0].to_string();
                line_contents = line_split[1].to_string();
            }
        }

        if !map.contains_key(&section) {
            map.insert(section.clone(), vec![]);
        }

        if line_contents.is_empty() {
            continue;
        }

        let numbers: Vec<usize> = line_contents
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        map.get_mut(&section).unwrap().push(numbers);
    }

    map
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ItemType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

pub type ItemTypeMapping = (ItemType, ItemType); // (From, To)

#[derive(Debug)]
pub struct AlmanacRange {
    pub destination_start: usize,
    pub source_start: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub maps: HashMap<ItemTypeMapping, Vec<AlmanacRange>>,
}

pub fn parse_raw_data_into_almanac(raw_data: HashMap<String, Vec<Vec<usize>>>) -> Almanac {
    let mut seeds = vec![];
    let mut maps: HashMap<ItemTypeMapping, Vec<AlmanacRange>> = HashMap::new();

    'raw_data_loop: for (item_type_string, numbers_vectors) in raw_data {
        let mut from_item_type = None;
        let mut to_item_type = None;

        match item_type_string.as_str() {
            "seeds" => {
                seeds = numbers_vectors.first().unwrap().clone();
                continue 'raw_data_loop;
            }
            "seed-to-soil map" => {
                from_item_type = Some(ItemType::Seed);
                to_item_type = Some(ItemType::Soil);
            }
            "soil-to-fertilizer map" => {
                from_item_type = Some(ItemType::Soil);
                to_item_type = Some(ItemType::Fertilizer);
            }
            "fertilizer-to-water map" => {
                from_item_type = Some(ItemType::Fertilizer);
                to_item_type = Some(ItemType::Water);
            }
            "water-to-light map" => {
                from_item_type = Some(ItemType::Water);
                to_item_type = Some(ItemType::Light);
            }
            "light-to-temperature map" => {
                from_item_type = Some(ItemType::Light);
                to_item_type = Some(ItemType::Temperature);
            }
            "temperature-to-humidity map" => {
                from_item_type = Some(ItemType::Temperature);
                to_item_type = Some(ItemType::Humidity);
            }
            "humidity-to-location map" => {
                from_item_type = Some(ItemType::Humidity);
                to_item_type = Some(ItemType::Location);
            }
            _ => {}
        };

        if from_item_type.is_none() || to_item_type.is_none() {
            panic!("Should be unreachable");
        }

        let from_item_type = from_item_type.unwrap();
        let to_item_type = to_item_type.unwrap();

        let mut almanac_ranges = vec![];
        for number_vector in numbers_vectors {
            almanac_ranges.push(AlmanacRange {
                destination_start: number_vector[0],
                source_start: number_vector[1],
                length: number_vector[2],
            });
        }

        maps.insert((from_item_type, to_item_type), almanac_ranges);
    }

    Almanac { seeds, maps }
}

const PATH: [ItemType; 8] = [
    ItemType::Seed,
    ItemType::Soil,
    ItemType::Fertilizer,
    ItemType::Water,
    ItemType::Light,
    ItemType::Temperature,
    ItemType::Humidity,
    ItemType::Location,
];

pub fn get_location_from_seed(
    seed: usize,
    maps: &HashMap<ItemTypeMapping, Vec<AlmanacRange>>,
) -> usize {
    let mut current_item = seed;

    for i in 0..PATH.len() - 1 {
        let from_item_type = PATH[i];
        let to_item_type = PATH[i + 1];
        let map_type = (from_item_type, to_item_type);

        let almanac_ranges = maps.get(&map_type).unwrap();

        current_item = get_destination(current_item, almanac_ranges);
    }

    current_item
}

fn get_destination(current_item: usize, almanac_ranges: &Vec<AlmanacRange>) -> usize {
    for almanac_range in almanac_ranges {
        if current_item >= almanac_range.source_start
            && current_item < almanac_range.source_start + almanac_range.length
        {
            let distance = current_item - almanac_range.source_start;
            return almanac_range.destination_start + distance;
        }
    }

    current_item
}
