use core::panic;
use std::collections::HashMap;

use day08::{parse_input_into_instruction_and_map_nodes, MapNodePointer};

fn main() {
    let (instructions, map_nodes) = parse_input_into_instruction_and_map_nodes("input/real.txt");
    let a_nodes = get_a_nodes(&map_nodes);

    print_nodes(&a_nodes);

    let finished_nodes =
        process_instructions_on_nodes_and_get_steps_count_for_each_node(&instructions, &a_nodes);

    println!("Finished nodes: {:?}", finished_nodes);
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

fn process_instructions_on_nodes_and_get_steps_count_for_each_node(
    instructions: &str,
    a_nodes: &Vec<MapNodePointer>,
) -> HashMap<usize, usize> {
    let mut finished_nodes = HashMap::new();
    let mut steps: usize = 0;
    let mut current_nodes = a_nodes.clone();

    let instruction_looping_iterator = instructions.chars().cycle();
    for next_instruction in instruction_looping_iterator {
        if finished_nodes.len() >= a_nodes.len() {
            break;
        }

        for index in 0..current_nodes.len() {
            process_instruction_on_node(
                steps,
                next_instruction,
                &mut current_nodes,
                index,
                &mut finished_nodes,
            );
        }

        steps += 1;
    }

    finished_nodes
}

fn process_instruction_on_node(
    steps: usize,
    next_instruction: char,
    current_nodes: &mut Vec<MapNodePointer>,
    index: usize,
    finished_nodes: &mut HashMap<usize, usize>,
) {
    if finished_nodes.contains_key(&index) {
        return;
    }

    let current_node = current_nodes.get_mut(index).unwrap();

    if current_node.borrow().label.ends_with('Z') {
        finished_nodes.insert(index, steps);
        return;
    }

    let next_node = follow_instruction(next_instruction, current_node);
    *current_node = next_node;
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
