use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

const EXPANSION_SIZE: usize = 2;

#[derive(PartialEq, Eq, Clone)]
pub enum SpaceObject {
    Empty,
    Galaxy,
}

#[derive(Clone)]
pub struct Space {
    pub object: SpaceObject,
    pub size: usize,
}

impl Space {
    pub fn is_empty(&self) -> bool {
        if self.object == SpaceObject::Empty {
            return true;
        }

        false
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

pub type SpaceGrid = Vec<Vec<Space>>;

pub type Coordinate = usize;
pub type Coordinates = (Coordinate, Coordinate);

pub fn parse_file_to_space_grid(path: &str) -> SpaceGrid {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut space_grid = vec![];

    for lines in lines {
        let line = lines.unwrap();

        let vector_of_chars = line.chars().collect::<Vec<char>>();

        let mut space_row = vec![];
        for character in vector_of_chars.iter() {
            match character {
                '.' => space_row.push(Space {
                    object: SpaceObject::Empty,
                    size: 1,
                }),
                '#' => space_row.push(Space {
                    object: SpaceObject::Galaxy,
                    size: 1,
                }),
                _ => panic!("Invalid character"),
            };
        }

        space_grid.push(space_row);
    }

    space_grid
}

pub fn get_expanded_space_grid(space_grid: SpaceGrid) -> SpaceGrid {
    let mut expanded_space_grid = vec![];

    fn contains_galaxy(vec: &Vec<Space>) -> bool {
        vec.iter().any(|r| r.is_not_empty())
    }

    for row in space_grid.iter() {
        let mut expand = false;
        if !contains_galaxy(row) {
            expand = true;
        }

        let mut cloned_row = row.clone();
        if expand {
            for space in cloned_row.iter_mut() {
                space.size = EXPANSION_SIZE;
            }
        }

        expanded_space_grid.push(cloned_row);
    }

    for column_index in 0..space_grid.first().unwrap().len() {
        let column = space_grid
            .iter()
            .map(|row| row[column_index].clone())
            .collect::<Vec<Space>>();

        let mut expand = false;
        if !contains_galaxy(&column) {
            expand = true;
        }

        if expand {
            for row in expanded_space_grid.iter_mut() {
                row[column_index].size = EXPANSION_SIZE;
            }
        }
    }

    expanded_space_grid
}

pub fn find_galaxies(space_grid: &SpaceGrid) -> Vec<Coordinates> {
    let mut coordinates = vec![];

    for (row_index, row) in space_grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if column.is_not_empty() {
                coordinates.push((column_index, row_index));
            }
        }
    }

    coordinates
}

pub fn calculate_distance_between_coordinates(a: &Coordinate, b: &Coordinate) -> usize {
    let x_distance = a.0.abs_diff(b.0);
    let y_distance = a.1.abs_diff(b.1);

    x_distance + y_distance
}
