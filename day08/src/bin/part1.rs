use day08::parse_input_into_instruction_and_map_nodes;

fn main() {
    let (instruction, map_nodes) = parse_input_into_instruction_and_map_nodes("input/real.txt");
    let first_node = map_nodes.get("AAA").unwrap().clone();

    let mut steps = 0;
    let mut current_node = first_node;

    let instruction_looping_iterator = instruction.chars().cycle();
    for next_instruction in instruction_looping_iterator {
        {
            if current_node.borrow().label == "ZZZ" {
                break;
            }
        }

        steps += 1;
        match next_instruction {
            'L' => {
                let next_node = current_node.as_ref().borrow().left.clone().unwrap();
                current_node = next_node;
            }
            'R' => {
                let next_node = current_node.as_ref().borrow().right.clone().unwrap();
                current_node = next_node;
            }
            _ => panic!("Invalid instruction"),
        };
    }

    println!("Steps: {}", steps);
}
