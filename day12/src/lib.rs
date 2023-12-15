use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct ConditionRecord {
    pub notation: String,
    pub damaged_grouping: Vec<u8>,
}

pub fn parse_file_to_condition_records(path: &str) -> Vec<ConditionRecord> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut condition_records = vec![];

    for lines in lines {
        let line = lines.unwrap();

        let split = line.split_whitespace().collect::<Vec<&str>>();
        let left = split[0];
        let right = split[1];

        let notation = left.replace(".", " ");

        let damaged_grouping = right
            .split(",")
            .map(|i| i.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        condition_records.push(ConditionRecord {
            notation,
            damaged_grouping,
        });
    }

    condition_records
}

pub fn get_possible_arragements(condition_record: &ConditionRecord) -> usize {
    if !condition_record.notation.contains('?') {
        return 0;
    }

    let mut possible_arrangements = 0;

    let question_marks_count = condition_record
        .notation
        .chars()
        .filter(|&c| c == '?')
        .count() as u32;

    for i in 0..(2 as usize).pow(question_marks_count) {
        let binary_notation = make_binary_notation(i, question_marks_count as usize);
        let try_notation = get_try_notation(&condition_record.notation, &binary_notation);
        let is_arrangement_valid =
            is_arrangement_valid(&try_notation, &condition_record.damaged_grouping);

        if is_arrangement_valid {
            possible_arrangements += 1;
        }
    }

    possible_arrangements
}

fn make_binary_notation(number: usize, length: usize) -> String {
    let mut binary_notation = format!("{:b}", number);

    if binary_notation.len() < length {
        let pad_length = length - binary_notation.len();
        let padding = "0".repeat(pad_length);
        binary_notation = format!("{}{}", padding, binary_notation);
    }

    binary_notation
}

fn get_try_notation(notation: &str, binary_notation: &str) -> String {
    let mut try_notation_chars = notation.chars().collect::<Vec<char>>();
    let binary_notation_chars = binary_notation.chars().collect::<Vec<char>>();

    let mut binary_index = 0;
    for char in try_notation_chars.iter_mut() {
        if *char != '?' {
            continue;
        }

        *char = match binary_notation_chars[binary_index] {
            '0' => ' ',
            '1' => '#',
            _ => panic!("Invalid binary notation"),
        };
        binary_index += 1;
    }

    try_notation_chars.iter().collect()
}

fn is_arrangement_valid(notation: &str, damaged_grouping: &Vec<u8>) -> bool {
    let split = notation
        .split_whitespace()
        .map(|str| str.trim())
        .filter(|str| str.len() > 0)
        .collect::<Vec<&str>>();

    if split.len() != damaged_grouping.len() {
        return false;
    }

    for index in 0..damaged_grouping.len() {
        if split[index].len() != damaged_grouping[index] as usize {
            return false;
        }
    }

    true
}
