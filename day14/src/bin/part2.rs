use std::collections::HashMap;

use day14::{calculate_load, convert_map_to_lines, parse_file_to_2d_map, spin_cycle_map, Map};

const CYCLES: usize = 1000000000;

fn main() {
    let map = parse_file_to_2d_map("input/real.txt");

    let (repeat_sample_hashmap, repeating_pattern_vec, last_spin_index) =
        find_repeating_pattern(map);

    let final_repeat_index = match find_final_repeat_index(last_spin_index, repeating_pattern_vec) {
        Some(final_repeat_index) => final_repeat_index,
        None => panic!("No final repeat index found"),
    };

    let map_at_final_cycle = match repeat_sample_hashmap.get(&final_repeat_index) {
        Some(map_at_final_cycle) => map_at_final_cycle.to_owned(),
        None => panic!("Final map cycle pattern not found"),
    };

    let final_cycle_lines = convert_map_to_lines(map_at_final_cycle);
    let mut loads = vec![];
    for line in final_cycle_lines {
        let load = calculate_load(line);
        loads.push(load);
    }

    let sum = loads.iter().sum::<usize>();
    println!("Sum: {}", sum);
}

fn find_repeating_pattern(mut map: Map) -> (HashMap<usize, Map>, Vec<usize>, usize) {
    let mut pattern_sample_hashmap = HashMap::new();
    let mut pattern_vec = vec![];
    let mut last_spin_index = 0;
    for spin_index in 0..1000 {
        let spin_index = spin_index + 1;

        map = spin_cycle_map(map);

        let repeat = pattern_sample_hashmap.get(&map);
        match repeat {
            Some(repeat_index) => {
                pattern_vec.push(*repeat_index);
                if check_pattern_vec_has_looped(&pattern_vec) {
                    last_spin_index = spin_index;
                    break;
                }
            }
            None => {
                pattern_sample_hashmap.insert(map.clone(), spin_index);
            }
        }
    }

    let pattern_vec = (&pattern_vec[0..pattern_vec.len() / 2]).to_vec();
    let mut flipped_sample_hashmap = HashMap::new();
    for (map_pattern, repeat_index) in pattern_sample_hashmap {
        flipped_sample_hashmap.insert(repeat_index, map_pattern);
    }

    (flipped_sample_hashmap, pattern_vec, last_spin_index)
}

fn check_pattern_vec_has_looped(pattern_vec: &Vec<usize>) -> bool {
    if pattern_vec.len() % 2 != 0 {
        return false;
    }

    let first_half = &pattern_vec[0..pattern_vec.len() / 2];
    let second_half = &pattern_vec[pattern_vec.len() / 2..];

    first_half == second_half
}

fn find_final_repeat_index(
    last_spin_index: usize,
    repeating_pattern_vec: Vec<usize>,
) -> Option<usize> {
    let mut spin_index = last_spin_index + 1;
    let mut final_repeat_index = None;
    for repeat_index in repeating_pattern_vec.iter().cycle() {
        if spin_index >= CYCLES {
            final_repeat_index = Some(*repeat_index);
            break;
        }

        spin_index += 1;
    }

    final_repeat_index
}
