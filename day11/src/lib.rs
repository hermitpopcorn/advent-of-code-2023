use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

pub fn get_expanded_space_grid(space_grid: SpaceGrid, expansion_size: usize) -> SpaceGrid {
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
                space.size = expansion_size;
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
                row[column_index].size = expansion_size;
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

pub fn calculate_distance_between_coordinates(
    space_grid: &SpaceGrid,
    a: &Coordinates,
    b: &Coordinates,
) -> usize {
    let stepped_spaces = pathfind_coordinates(a, b);

    let mut distance = 0;
    for stepped_space in stepped_spaces {
        let space = &space_grid[stepped_space.1][stepped_space.0];
        distance += space.size;
    }

    distance
}

fn pathfind_coordinates(start: &Coordinates, target: &Coordinates) -> Vec<Coordinates> {
    let x_moves = pathfind_axis(start.0, target.0);
    let y_moves = pathfind_axis(start.1, target.1);

    let mut current_position = start.clone();
    let mut stepped_spaces = vec![];

    if let Some(x_moves) = x_moves {
        for x_move in x_moves {
            current_position = (x_move, current_position.1);
            stepped_spaces.push(current_position.clone());
        }
    }

    if let Some(y_moves) = y_moves {
        for y_move in y_moves {
            current_position = (current_position.0, y_move);
            stepped_spaces.push(current_position.clone());
        }
    }

    stepped_spaces
}

fn pathfind_axis(a: usize, b: usize) -> Option<Vec<usize>> {
    if a == b {
        return None;
    }

    let mut path = vec![];

    if a < b {
        for i in (a + 1)..=b {
            path.push(i);
        }
    } else {
        for i in b..=(a - 1) {
            path.insert(0, i);
        }
    }

    Some(path)
}
