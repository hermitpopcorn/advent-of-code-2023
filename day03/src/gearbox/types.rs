use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Write},
    rc::Rc,
};

pub struct Gearbox {
    gear_matrix: GearMatrix,
    gear_line_length: usize,
    part_number_sequences: PartNumberSequences,
    symbol_coordinates: SymbolCoordinates,
}

impl Gearbox {
    pub fn new(
        gear_matrix: GearMatrix,
        gear_line_length: usize,
        part_number_sequences: PartNumberSequences,
        symbol_coordinates: SymbolCoordinates,
    ) -> Gearbox {
        Gearbox {
            gear_matrix,
            gear_line_length,
            part_number_sequences,
            symbol_coordinates,
        }
    }

    pub fn get_part_numbers(&self) -> HashSet<Rc<PartNumberSequence>> {
        let mut part_numbers = HashSet::new();

        for symbol_coordinate in &self.symbol_coordinates {
            let surrounding_coordinates =
                get_surrounding_coordinates(*symbol_coordinate, self.gear_line_length);

            for possible_coordinate in surrounding_coordinates {
                match self.part_number_sequences.get(&possible_coordinate) {
                    Some(sequence) => {
                        part_numbers.insert(sequence.clone());
                    }
                    None => {}
                }
            }
        }

        part_numbers
    }
}

fn get_surrounding_coordinates(coordinate: Coordinate, line_length: usize) -> Vec<Coordinate> {
    let mut surrounding_coordinates = vec![];

    let top = std::cmp::max(0, (coordinate.y as isize) - 1);
    let left = std::cmp::max(0, (coordinate.x as isize) - 1);
    let right = std::cmp::min(line_length as isize, (coordinate.x as isize) + 1);
    let bottom = std::cmp::min(line_length as isize, (coordinate.y as isize) + 1);

    for x in left..right + 1 {
        for y in top..bottom + 1 {
            surrounding_coordinates.push(Coordinate {
                x: x as usize,
                y: y as usize,
            })
        }
    }

    surrounding_coordinates
}

impl Display for Gearbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.gear_matrix {
            let _ = f.write_str("> ");

            for k in i {
                let c = match k {
                    GearType::Number(char) => char::from_digit(*char as u32, 10).unwrap(),
                    GearType::Symbol(char) => *char,
                    GearType::Nothing => '.',
                };
                let _ = f.write_char(c);
            }
            let _ = f.write_char('\n');
        }

        Ok(())
    }
}

pub enum GearType {
    Number(usize),
    Symbol(char),
    Nothing,
}

#[derive(PartialEq, Eq, Hash)]
pub struct PartNumberSequence {
    identifier_coordinate: Coordinate,
    pub value: usize,
}

impl PartNumberSequence {
    pub fn new(identifier_coordinate: Coordinate, value: usize) -> PartNumberSequence {
        PartNumberSequence {
            identifier_coordinate,
            value,
        }
    }
}

impl Debug for PartNumberSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str("PartNumberSequence { Identifier: ");
        let _ = f.write_fmt(format_args!("{:?}", self.identifier_coordinate));
        let _ = f.write_str(", Value: ");
        let _ = f.write_fmt(format_args!("{:?}", self.value));
        let _ = f.write_str(" }");

        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_char('(');
        let _ = f.write_str(&self.x.to_string());
        let _ = f.write_char(',');
        let _ = f.write_str(&self.y.to_string());
        let _ = f.write_char(')');

        Ok(())
    }
}

pub type GearMatrix = Vec<Vec<GearType>>;
pub type PartNumberSequences = HashMap<Coordinate, Rc<PartNumberSequence>>;
pub type SymbolCoordinates = Vec<Coordinate>;
