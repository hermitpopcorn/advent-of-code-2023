use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub type SpaceGrid = Vec<Vec<char>>;

pub type Coordinate = (usize, usize);

pub fn parse_file_to_space_grid(path: &str) -> SpaceGrid {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut space_grid = vec![];

    for lines in lines {
        let line = lines.unwrap();

        let vector_of_chars = line.chars().collect::<Vec<char>>();

        space_grid.push(vector_of_chars);
    }

    space_grid
}

pub fn get_expanded_space_grid(space_grid: SpaceGrid) -> SpaceGrid {
    let mut expanded_space_grid = vec![];

    for row in space_grid.iter() {
        let mut expand = false;
        if !row.contains(&'#') {
            expand = true;
        }

        expanded_space_grid.push(row.clone());
        if expand {
            expanded_space_grid.push(row.clone());
        }
    }

    let mut column_expansions = 0;
    for column_index in 0..space_grid.first().unwrap().len() {
        let column = space_grid
            .iter()
            .map(|row| row[column_index])
            .collect::<Vec<char>>();

        let mut expand = false;
        if !column.contains(&'#') {
            expand = true;
        }

        if expand {
            column_expansions += 1;
            for row in expanded_space_grid.iter_mut() {
                row.insert(column_index + column_expansions, '.');
            }
        }
    }

    expanded_space_grid
}

pub fn find_galaxies(space_grid: &SpaceGrid) -> Vec<Coordinate> {
    let mut coordinates = vec![];

    for (row_index, row) in space_grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if *column == '#' {
                // Plus one because I don't want them to be 0-indexed
                coordinates.push((column_index + 1, row_index + 1));
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
