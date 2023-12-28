use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThingType {
    Nothing,
    Roller,
    Stopper,
}

#[derive(Debug, Clone)]
pub struct Thing {
    pub what: ThingType,
    pub location: usize,
}

pub type Line = Vec<Thing>;

pub fn parse_file_to_lines(path: &str) -> Vec<Line> {
    let two_dimensional_char_map = parse_file_to_2d_char_map(path);
    let lines = parse_2d_char_map_to_lines(two_dimensional_char_map);
    lines
}

fn parse_file_to_2d_char_map(path: &str) -> Vec<Vec<char>> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut two_dimensional_char_map = vec![];
    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        two_dimensional_char_map.push(line.chars().collect::<Vec<char>>());
    }

    two_dimensional_char_map
}

fn parse_2d_char_map_to_lines(two_dimensional_char_map: Vec<Vec<char>>) -> Vec<Line> {
    let mut lines = vec![];

    let y_length = two_dimensional_char_map.len();
    let x_length = two_dimensional_char_map.first().unwrap().len();

    for x in 0..x_length {
        let mut line = vec![];
        for y in 0..y_length {
            let character = two_dimensional_char_map[y][x];

            let thing_type = match character {
                '.' => ThingType::Nothing,
                'O' => ThingType::Roller,
                '#' => ThingType::Stopper,
                _ => panic!("Unknown character"),
            };
            line.push(Thing {
                what: thing_type,
                location: y,
            });
        }
        lines.push(line);
    }

    lines
}

pub fn tilt_line(line: Line) -> Line {
    let mut tilted_line = line.clone();

    let mut process_stopper = |index: Option<usize>, roller_stacks: &mut Vec<&Thing>| {
        let get_location = |index: Option<usize>, stack_index: usize| {
            if index.is_some() {
                index.unwrap() + (stack_index + 1)
            } else {
                stack_index
            }
        };

        let mut inserted_locations: Vec<usize> = vec![];
        for (stack_index, _) in roller_stacks.iter().enumerate() {
            let location = get_location(index, stack_index);
            tilted_line[location].what = ThingType::Roller;
            inserted_locations.push(location);
        }

        for stack_roller in roller_stacks.iter() {
            if !inserted_locations.contains(&stack_roller.location) {
                tilted_line[stack_roller.location].what = ThingType::Nothing;
            }
        }

        roller_stacks.clear();
    };

    let mut roller_stacks = vec![];
    for index in (0..line.len()).rev() {
        let thing = &line[index];

        if thing.what == ThingType::Nothing {
            continue;
        }

        if thing.what == ThingType::Roller {
            roller_stacks.push(thing);
            continue;
        }

        if thing.what == ThingType::Stopper {
            process_stopper(Some(index), &mut roller_stacks);
        }
    }

    if roller_stacks.len() > 0 {
    process_stopper(None, &mut roller_stacks);
    }

    tilted_line
}

pub fn calculate_load(line: Line) -> usize {
    let mut total_load = 0;
    let mut current_load_size = 1;
    for index in (0..line.len()).rev() {
        let thing = &line[index];
        if thing.what == ThingType::Roller {
            total_load += current_load_size;
        }

        current_load_size += 1;
    }

    total_load
}
