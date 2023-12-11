use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn flip(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
pub struct Pipe {
    pub directions: [Direction; 2],
}

impl Pipe {
    pub fn has_direction(&self, direction: &Direction) -> bool {
        self.directions[0] == *direction || self.directions[1] == *direction
    }

    pub fn get_exit_direction_and_coordinate_from_entry_direction(
        &self,
        current_coordinate: &Coordinate,
        direction: &Direction,
    ) -> Result<(Direction, Coordinate), ()> {
        let exit_direction = self.get_exit_direction_from_entry_direction(direction)?;

        let exit_coordinate = self.get_exit_coordinate_from_coordinate_and_exit_direction(
            current_coordinate,
            &exit_direction,
        );

        Ok((exit_direction, exit_coordinate))
    }

    pub fn get_exit_direction_from_entry_direction(
        &self,
        direction: &Direction,
    ) -> Result<Direction, ()> {
        if *direction == self.directions[0] {
            return Ok(self.directions[1]);
        } else if *direction == self.directions[1] {
            return Ok(self.directions[0]);
        }

        Err(())
    }

    pub fn get_exit_coordinate_from_coordinate_and_exit_direction(
        &self,
        coordinate: &Coordinate,
        exit_direction: &Direction,
    ) -> Coordinate {
        let (x, y) = *coordinate;

        match exit_direction {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

pub type Grid = Vec<Vec<char>>;
pub type Coordinate = (usize, usize);

pub fn parse_file_to_grid_of_characters(path: &str) -> Grid {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut grid = vec![];

    for lines in lines {
        let line = lines.unwrap();

        let vector_of_chars = line.chars().collect::<Vec<char>>();

        grid.push(vector_of_chars);
    }

    grid
}

pub fn get_starting_point(grid: &Grid) -> Coordinate {
    for y in 0..grid.len() {
        let row = &grid[y];

        for x in 0..row.len() {
            if row[x] == 'S' {
                return (x, y);
            }
        }
    }

    panic!("Starting point not found");
}

pub fn get_surrounding_coordinates(coordinate: Coordinate) -> HashMap<Direction, Coordinate> {
    let mut surrounding_coordinates = HashMap::new();

    let (x, y) = coordinate;

    surrounding_coordinates.insert(Direction::North, (x, y - 1));
    surrounding_coordinates.insert(Direction::East, (x + 1, y));
    surrounding_coordinates.insert(Direction::South, (x, y + 1));
    surrounding_coordinates.insert(Direction::West, (x - 1, y));

    surrounding_coordinates
}

pub fn get_grid_character(grid: &Grid, coordinate: &Coordinate) -> char {
    let (x, y) = coordinate;

    grid[*y][*x]
}

pub fn translate_character_to_pipe(character: char) -> Option<Pipe> {
    match character {
        '|' => Some(Pipe {
            directions: [Direction::North, Direction::South],
        }),
        '-' => Some(Pipe {
            directions: [Direction::East, Direction::West],
        }),
        'L' => Some(Pipe {
            directions: [Direction::North, Direction::East],
        }),
        'J' => Some(Pipe {
            directions: [Direction::North, Direction::West],
        }),
        '7' => Some(Pipe {
            directions: [Direction::South, Direction::West],
        }),
        'F' => Some(Pipe {
            directions: [Direction::South, Direction::East],
        }),
        '.' | 'S' => None,
        _ => panic!("Invalid character"),
    }
}

pub fn can_connect_to_pipe_in_direction(pipe: &Pipe, direction: &Direction) -> bool {
    pipe.has_direction(direction)
}
