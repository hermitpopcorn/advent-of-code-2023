use day10::{
    pipegrid::{get_starting_point, parse_file_to_grid_of_characters},
    trace::trace_for_loop_in_grid,
};

fn main() {
    let grid = parse_file_to_grid_of_characters("input/real.txt");

    let starting_point = get_starting_point(&grid);

    let result = trace_for_loop_in_grid(grid, starting_point);

    println!("Answer: {}", result.history.len() / 2);
}
