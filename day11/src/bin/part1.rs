use std::collections::HashMap;

use day11::{
    calculate_distance_between_coordinates, find_galaxies, get_expanded_space_grid,
    parse_file_to_space_grid,
};

fn main() {
    let space_grid = parse_file_to_space_grid("input/real.txt");

    let expanded_space_grid = get_expanded_space_grid(space_grid, 2);

    let galaxy_coordinates = find_galaxies(&expanded_space_grid);

    let mut galaxy_distances = HashMap::new();
    for (index, galaxy_coordinate) in galaxy_coordinates.iter().enumerate() {
        for comparison_galaxy_coordinate in &galaxy_coordinates[index..galaxy_coordinates.len()] {
            if galaxy_coordinate == comparison_galaxy_coordinate {
                continue;
            }

            let distance = calculate_distance_between_coordinates(
                &expanded_space_grid,
                galaxy_coordinate,
                comparison_galaxy_coordinate,
            );

            galaxy_distances.insert((galaxy_coordinate, comparison_galaxy_coordinate), distance);
        }
    }

    let sum_of_galaxy_distances = galaxy_distances.values().sum::<usize>();
    println!("Sum of shortest paths: {}", sum_of_galaxy_distances);
}
