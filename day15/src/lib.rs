use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ascii_converter::string_to_decimals;

pub fn parse_file_to_sequences(path: &str) -> Vec<String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut sequences = vec![];

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        sequences.append(
            &mut line
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>(),
        );
    }

    sequences
}

pub fn hash_sequence(sequence: &str) -> usize {
    let mut hash = 0;

    for c_index in 0..sequence.len() {
        let c = &sequence[c_index..c_index + 1];
        let decimal = string_to_decimals(c).unwrap().first().unwrap().to_owned();
        hash = hash + usize::from(decimal);
        hash = hash * 17;
        hash = hash.rem_euclid(256);
    }

    hash
}
