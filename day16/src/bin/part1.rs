use day16::{Direction, TileMap};

fn main() {
    let mut tile_map = TileMap::from_file_path("input/real.txt");

    tile_map.traverse(0, 0, Direction::Right);

    println!("{}", tile_map.get_energized_tiles_count());
}
