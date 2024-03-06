use day16::{Direction, TileMap};

fn main() {
    let base_tile_map = TileMap::from_file_path("input/real.txt");

    let mut energized_tiles_counts = vec![];
    for y in 0..base_tile_map.vertical_length {
        for x in 0..base_tile_map.horizontal_length {
            let max_x = base_tile_map.horizontal_length - 1;
            let max_y = base_tile_map.vertical_length - 1;

            // if not edge, skip
            if x != 0 && y != 0 && x != max_x && y != max_y {
                continue;
            }

            let start_directions = get_start_directions(x, y, max_x, max_y);

            start_directions.into_iter().for_each(|direction| {
                let mut cloned_tile_map = base_tile_map.clone();
                cloned_tile_map.traverse(x, y, direction);
                energized_tiles_counts.push(cloned_tile_map.get_energized_tiles_count());
            });
        }
    }

    println!("{}", energized_tiles_counts.iter().max().unwrap_or(&0));
}

fn get_start_directions(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<Direction> {
    let mut start_directions = vec![];

    if x == 0 {
        start_directions.push(Direction::Right);
    }
    if x == max_x {
        start_directions.push(Direction::Left);
    }
    if y == 0 {
        start_directions.push(Direction::Down);
    }
    if y == max_y {
        start_directions.push(Direction::Up);
    }

    start_directions
}
