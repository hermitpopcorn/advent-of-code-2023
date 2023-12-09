use core::panic;
use std::collections::HashMap;

use day08::{parse_input_into_instruction_and_map_nodes, MapNodePointer};

fn main() {
    let (instructions, map_nodes) = parse_input_into_instruction_and_map_nodes("input/real.txt");
    let a_nodes = get_a_nodes(&map_nodes);

    print_nodes(&a_nodes);

    let mut steps: usize = 0;
    let mut current_nodes = a_nodes.clone();
    let mut best_finished_count_so_far = 0;

    let instruction_looping_iterator = instructions.chars().cycle();
    for next_instruction in instruction_looping_iterator {
        let finished_nodes = get_finished_nodes(&current_nodes);
        let all_finished = finished_nodes.values().all(|f| *f);
        if all_finished {
            break;
        }

        let finished_nodes_count = finished_nodes.values().filter(|f| **f).count();
        if finished_nodes_count > best_finished_count_so_far {
            for node in &current_nodes {
                let label = { node.borrow().label.clone() };
                let status = if label.ends_with('Z') { "Y" } else { "" };
                println!("{} {}", label, status);
            }
        }

        print_best_finished_count_so_far(steps, &mut best_finished_count_so_far, &finished_nodes);

        have_nodes_follow_instruction(next_instruction, &mut current_nodes);

        steps += 1;
    }

    println!("Steps: {}", steps);
}

fn get_a_nodes(map_nodes: &HashMap<String, MapNodePointer>) -> Vec<MapNodePointer> {
    let labels = map_nodes
        .keys()
        .into_iter()
        .filter(|label| label.ends_with('A'))
        .collect::<Vec<&String>>();

    let mut a_nodes = vec![];
    for label in labels {
        a_nodes.push(map_nodes.get(label).unwrap().clone());
    }

    a_nodes
}

#[allow(dead_code)]
fn print_nodes(nodes: &Vec<MapNodePointer>) {
    for node in nodes {
        let node = node.borrow();
        println!("{}", node);
    }
}

fn get_finished_nodes(nodes: &Vec<MapNodePointer>) -> HashMap<usize, bool> {
    let mut finished_nodes = HashMap::new();

    for (index, node) in nodes.iter().enumerate() {
        let current_label = node.borrow().label.clone();
        let is_finished = current_label.ends_with('Z');
        finished_nodes.insert(index, is_finished);
    }

    finished_nodes
}

#[allow(dead_code)]
fn print_best_finished_count_so_far(
    steps: usize,
    best_finished_count_so_far: &mut usize,
    finished_nodes: &HashMap<usize, bool>,
) {
    let count = finished_nodes.values().filter(|f| **f).count();
    if count > *best_finished_count_so_far {
        *best_finished_count_so_far = count;
        println!("Finished: {} | Steps: {}", count, steps);
    }
}

fn have_nodes_follow_instruction(instruction: char, nodes: &mut Vec<MapNodePointer>) {
    (0..nodes.len()).into_iter().for_each(|index| {
        let current_node = nodes.get_mut(index).unwrap();
        let next_node = follow_instruction(instruction, current_node);

        *current_node = next_node;
    });
}

fn follow_instruction(instruction: char, node: &MapNodePointer) -> MapNodePointer {
    let node = node.borrow();

    let next = match instruction {
        'L' => node.left.clone(),
        'R' => node.right.clone(),
        _ => panic!("Invalid instruction"),
    };

    next.unwrap()
}
