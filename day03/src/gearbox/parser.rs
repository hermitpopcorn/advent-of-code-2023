use std::{collections::HashMap, rc::Rc};

use super::types::{
    Coordinate, GearMatrix, GearType, Gearbox, PartNumberSequence, PartNumberSequences,
    SymbolCoordinates,
};

pub fn parse_gearbox(input: &str) -> Gearbox {
    let mut gear_matrix: GearMatrix = vec![];
    let mut part_number_sequences: PartNumberSequences = HashMap::new();
    let mut symbol_coordinates: SymbolCoordinates = vec![];

    let input_per_line_collection: Vec<&str> = input.split("\n").collect();
    let gear_line_length = input_per_line_collection.first().unwrap().len();

    for (index, line) in input_per_line_collection.iter().enumerate() {
        if line.len() < 1 {
            continue;
        }

        let (line_gear_matrix, line_part_number_sequences, mut line_symbol_coordinates) =
            parse_gearbox_line(line, index);

        gear_matrix.push(line_gear_matrix);

        for (coords, sequence) in line_part_number_sequences.iter() {
            part_number_sequences.insert(*coords, sequence.clone());
        }

        symbol_coordinates.append(&mut line_symbol_coordinates);
    }

    Gearbox::new(
        gear_matrix,
        gear_line_length,
        part_number_sequences,
        symbol_coordinates,
    )
}

fn parse_gearbox_line(
    line: &str,
    line_index: usize,
) -> (Vec<GearType>, PartNumberSequences, SymbolCoordinates) {
    let mut line_gear_matrix: Vec<GearType> = vec![];
    let mut line_part_number_sequences: PartNumberSequences = HashMap::new();
    let mut sequence_recorder_string: String = String::from("");
    let mut line_symbol_coordinates: SymbolCoordinates = vec![];

    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len() + 1 {
        let x_pos = i;
        let y_pos = line_index;
        let coordinate = Coordinate::new(x_pos, y_pos);

        match chars.get(i) {
            Some(char) => {
                if char.is_numeric() {
                    let digit = char.to_digit(10).unwrap();
                    let digit = usize::try_from(digit).unwrap();
                    line_gear_matrix.push(GearType::Number(digit));
                    sequence_recorder_string.push(*char);
                    continue;
                }

                if char.eq(&'.') {
                    line_gear_matrix.push(GearType::Nothing);
                } else {
                    line_gear_matrix.push(GearType::Symbol(*char));
                    line_symbol_coordinates.push((*char, coordinate.clone()));
                }
            }
            None => {}
        };

        let sequence_length = sequence_recorder_string.len();
        if sequence_length < 1 {
            continue;
        }
        let part_number_sequence =
            create_part_number_sequence(&sequence_recorder_string, coordinate.clone());
        sequence_recorder_string.clear();

        let part_number_sequence = Rc::new(part_number_sequence);
        for k in i - sequence_length..i {
            let x_pos = usize::try_from(k).unwrap();
            let y_pos = line_index;
            line_part_number_sequences
                .insert(Coordinate::new(x_pos, y_pos), part_number_sequence.clone());
        }
    }

    (
        line_gear_matrix,
        line_part_number_sequences,
        line_symbol_coordinates,
    )
}

fn create_part_number_sequence(
    sequence_string: &str,
    coordinate: Coordinate,
) -> PartNumberSequence {
    PartNumberSequence::new(coordinate, sequence_string.parse().unwrap())
}
