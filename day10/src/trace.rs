use crate::pipegrid::{
    get_grid_character, get_surrounding_coordinates, translate_character_to_pipe, Coordinate,
    Direction, Grid,
};

pub struct TraceResult {
    pub result: TraceResultType,
    pub history: Vec<Coordinate>,
}

#[derive(PartialEq, Eq)]
pub enum TraceResultType {
    DeadEnd,
    BackToStart,
}

impl TraceResult {
    pub fn dead_end(history: Vec<Coordinate>) -> Self {
        Self {
            result: TraceResultType::DeadEnd,
            history,
        }
    }
    pub fn back_to_start(history: Vec<Coordinate>) -> Self {
        Self {
            result: TraceResultType::BackToStart,
            history,
        }
    }
}

pub fn trace_for_loop_in_grid(grid: Grid, starting_coordinate: Coordinate) -> TraceResult {
    let surrounding_coordinates = get_surrounding_coordinates(starting_coordinate);

    for (direction, coordinate) in surrounding_coordinates.iter() {
        let trace = trace_branch(&grid, *coordinate, *direction, vec![]);
        if trace.result == TraceResultType::BackToStart {
            return trace;
        }
    }

    panic!("Loop not found")
}

fn trace_branch(
    grid: &Grid,
    coordinate: Coordinate,
    coming_from_direction: Direction,
    mut history: Vec<Coordinate>,
) -> TraceResult {
    history.push(coordinate);

    let in_direction = coming_from_direction.flip();

    let grid_character = get_grid_character(grid, &coordinate);
    if grid_character == 'S' {
        return TraceResult::back_to_start(history);
    }
    if grid_character == '.' {
        return TraceResult::dead_end(history);
    }

    let pipe = translate_character_to_pipe(grid_character);
    if pipe.is_none() {
        return TraceResult::dead_end(history);
    }
    let pipe = pipe.unwrap();

    let exit =
        pipe.get_exit_direction_and_coordinate_from_entry_direction(&coordinate, &in_direction);
    if exit.is_err() {
        return TraceResult::dead_end(history);
    }
    let (exit_direction, exit_coordinate) = exit.unwrap();

    trace_branch(grid, exit_coordinate, exit_direction, history)
}
