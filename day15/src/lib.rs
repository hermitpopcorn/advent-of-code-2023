use std::{
    collections::HashMap,
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

#[derive(PartialEq, Eq)]
pub enum Operation {
    Equals,
    Dash,
}

pub struct Step {
    pub label: String,
    pub operation: Operation,
    pub focal_length: Option<u8>,
}

pub struct LensBox {
    pub indexes: HashMap<String, usize>,
    pub lenses: Vec<u8>,
}
impl LensBox {
    pub fn new() -> LensBox {
        LensBox {
            indexes: HashMap::new(),
            lenses: vec![],
        }
    }

    pub fn remove(&mut self, label: &str) {
        let index = self.indexes.get(label);
        if index.is_none() {
            return;
        }
        let index = index.unwrap();

        self.lenses.remove(*index);
        self.remove_index(label);
    }

    fn remove_index(&mut self, label: &str) {
        let removed_index = self.indexes.remove(label).unwrap();
        for (_, index) in self.indexes.iter_mut() {
            if *index > removed_index {
                *index = *index - 1;
            }
        }
    }

    pub fn insert(&mut self, label: String, focal_length: u8) {
        match self.indexes.get(&label) {
            Some(existing_index) => {
                let lens = self.lenses.get_mut(*existing_index).unwrap();
                *lens = focal_length;
            }
            None => {
                let index = self.lenses.len();
                self.lenses.push(focal_length);
                self.indexes.insert(label, index);
            }
        }
    }
}

pub fn get_step_from_sequence(sequence: &str) -> Step {
    let mut label = String::new();
    let mut operation = None;
    let mut focal_length = None;

    let mut parse_char = |c_str: &str| match c_str {
        "-" => {
            operation = Some(Operation::Dash);
        }
        "=" => {
            operation = Some(Operation::Equals);
        }
        _ => match operation {
            Some(_) => {
                let number_str = c_str.parse().unwrap();
                focal_length = Some(number_str);
            }
            None => label.push_str(c_str),
        },
    };

    for c_index in 0..sequence.len() {
        let c_str = &sequence[c_index..c_index + 1];
        parse_char(c_str);
    }

    if operation.is_none() {
        panic!();
    }

    Step {
        label,
        operation: operation.unwrap(),
        focal_length,
    }
}
