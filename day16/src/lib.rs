use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum NewDirection {
    NoChange,
    Reflected(Direction),
    Split(Vec<Direction>),
}

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    EmptySpace,
    SlashMirror,
    BackslashMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnknownCharError {
    char: char,
}

impl TileType {
    pub fn from_char(c: &char) -> Result<Self, UnknownCharError> {
        let s = c.to_string();
        match s.as_str() {
            "." => Ok(Self::EmptySpace),
            "/" => Ok(Self::SlashMirror),
            "\\" => Ok(Self::BackslashMirror),
            "|" => Ok(Self::VerticalSplitter),
            "-" => Ok(Self::HorizontalSplitter),
            _ => Err(UnknownCharError { char: c.to_owned() }),
        }
    }

    pub fn get_new_direction_from(&self, direction: &Direction) -> NewDirection {
        match self {
            Self::EmptySpace => NewDirection::NoChange,
            Self::SlashMirror => match direction {
                Direction::Up => NewDirection::Reflected(Direction::Right),
                Direction::Down => NewDirection::Reflected(Direction::Left),
                Direction::Left => NewDirection::Reflected(Direction::Down),
                Direction::Right => NewDirection::Reflected(Direction::Up),
            },
            Self::BackslashMirror => match direction {
                Direction::Up => NewDirection::Reflected(Direction::Left),
                Direction::Down => NewDirection::Reflected(Direction::Right),
                Direction::Left => NewDirection::Reflected(Direction::Up),
                Direction::Right => NewDirection::Reflected(Direction::Down),
            },
            Self::VerticalSplitter => match direction {
                Direction::Up => NewDirection::NoChange,
                Direction::Down => NewDirection::NoChange,
                Direction::Left => NewDirection::Split(vec![Direction::Up, Direction::Down]),
                Direction::Right => NewDirection::Split(vec![Direction::Up, Direction::Down]),
            },
            Self::HorizontalSplitter => match direction {
                Direction::Up => NewDirection::Split(vec![Direction::Left, Direction::Right]),
                Direction::Down => NewDirection::Split(vec![Direction::Left, Direction::Right]),
                Direction::Left => NewDirection::NoChange,
                Direction::Right => NewDirection::NoChange,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TileMap {
    pub tile_map: Vec<Vec<TileType>>,
    pub energy_map: Vec<Vec<usize>>,
    pub horizontal_length: usize,
    pub vertical_length: usize,
    tread_path: TreadPath,
}

#[derive(Debug)]
pub struct OutOfBoundsError;

type TreadHistory = ((usize, usize), Direction);
type TreadPath = Vec<TreadHistory>;

impl TileMap {
    pub fn from_file_path(path: &str) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut tile_map = vec![];
        let mut energy_map = vec![];
        let mut horizontal_length = 0;

        for line in lines {
            let line = line.unwrap();

            if line.is_empty() {
                break;
            }

            let mut tile_line = vec![];
            let mut energy_line = vec![];

            for c in line.chars() {
                let tile_type = TileType::from_char(&c).unwrap();
                tile_line.push(tile_type);
                energy_line.push(0);
                horizontal_length = tile_line.len();
            }

            tile_map.push(tile_line);
            energy_map.push(energy_line);
        }

        let vertical_length = tile_map.len();

        Self {
            tile_map,
            energy_map,
            horizontal_length,
            vertical_length,
            tread_path: vec![],
        }
    }

    pub fn get_tile_type_at(&self, x: usize, y: usize) -> TileType {
        let tile_type = self.tile_map.get(y).unwrap().get(x).unwrap();
        tile_type.clone()
    }

    pub fn traverse(&mut self, x: usize, y: usize, direction: Direction) {
        let history = ((x, y), direction);
        if self.tread_path.contains(&history) {
            return;
        }

        self.energize_at(x, y);
        self.tread_path.push(history);
        let tile_type = self.get_tile_type_at(x, y);
        let new_direction = tile_type.get_new_direction_from(&direction);

        match new_direction {
            NewDirection::NoChange => {
                let next_coords = self.get_next_coordinate(x, y, &direction);
                if next_coords.is_err() {
                    return;
                }
                let next_coords = next_coords.unwrap();
                self.traverse(next_coords.0, next_coords.1, direction);
            }
            NewDirection::Reflected(new_direction) => {
                let next_coords = self.get_next_coordinate(x, y, &new_direction);
                if next_coords.is_err() {
                    return;
                }
                let next_coords = next_coords.unwrap();
                self.traverse(next_coords.0, next_coords.1, new_direction);
            }
            NewDirection::Split(split_directions) => {
                split_directions.iter().for_each(|new_direction| {
                    let next_coords = self.get_next_coordinate(x, y, new_direction);
                    if next_coords.is_err() {
                        return;
                    }
                    let next_coords = next_coords.unwrap();

                    self.traverse(next_coords.0, next_coords.1, new_direction.clone());
                });
            }
        };
    }

    fn get_next_coordinate(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Result<(usize, usize), OutOfBoundsError> {
        let isize_x: isize = x.try_into().unwrap();
        let isize_y: isize = y.try_into().unwrap();
        let (next_x, next_y): (isize, isize) = match direction {
            Direction::Up => (isize_x, isize_y - 1),
            Direction::Down => (isize_x, isize_y + 1),
            Direction::Left => (isize_x - 1, isize_y),
            Direction::Right => (isize_x + 1, isize_y),
        };

        if next_x < 0
            || next_y < 0
            || next_x >= self.horizontal_length.try_into().unwrap()
            || next_y >= self.vertical_length.try_into().unwrap()
        {
            return Err(OutOfBoundsError);
        }

        Ok((next_x.try_into().unwrap(), next_y.try_into().unwrap()))
    }

    fn energize_at(&mut self, x: usize, y: usize) {
        let tile_energy = self.energy_map.get_mut(y).unwrap().get_mut(x).unwrap();
        *tile_energy += 1;
    }

    pub fn get_energized_tiles_count(&self) -> usize {
        let mut energized_tiles_count = 0;
        for y_vec in self.energy_map.iter() {
            for x in y_vec.iter() {
                if *x > 0 {
                    energized_tiles_count += 1;
                }
            }
        }
        energized_tiles_count
    }
}
