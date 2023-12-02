fn main() {
    let inputs = std::fs::read_to_string("input/real.txt").unwrap();
    let inputs: Vec<&str> = inputs.split("\n").collect();

    let mut values = vec![];
    for input in inputs {
        if input.len() < 1 {
            continue;
        }

        let value = parse(input);
        values.push(value);

        println!("{} <- {}", value, input);
    }

    let answer: u32 = values.iter().sum();
    println!("Answer: {}", answer);
}

fn parse(input: &str) -> u32 {
    let game = parse_game(input);

    let game_sets = parse_game_sets_string_into_cube_arrays(&game);

    let least_numbers = get_minimum_cube_amounts(game_sets);

    return least_numbers[0] * least_numbers[1] * least_numbers[2];
}

fn parse_game(input: &str) -> String {
    let split: Vec<&str> = input.split(": ").collect();

    let game_string = String::from(*split.get(1).unwrap());

    game_string
}

fn parse_game_sets_string_into_cube_arrays(input: &str) -> Vec<[u32; 3]> {
    let mut sets = vec![];

    let split: Vec<&str> = input.split("; ").collect();
    for set_str in split {
        let set = parse_set_string_into_cube_array(set_str);
        sets.push(set);
    }

    sets
}

fn parse_set_string_into_cube_array(input: &str) -> [u32; 3] {
    let mut cubes = [0, 0, 0];
    let split: Vec<&str> = input.split(" ").collect();

    for index in 0..split.len() / 2 {
        let amount_index = index * 2;
        let color_index = index * 2 + 1;
        let amount: u32 = String::from(*split.get(amount_index).unwrap())
            .parse()
            .unwrap();
        let color: String = String::from(*split.get(color_index).unwrap());

        if color.contains("red") {
            cubes[0] = amount;
        } else if color.contains("green") {
            cubes[1] = amount;
        } else if color.contains("blue") {
            cubes[2] = amount;
        }
    }

    cubes
}

fn get_minimum_cube_amounts(sets: Vec<[u32; 3]>) -> [u32; 3] {
    let mut minimum_cube_amounts = [0, 0, 0];

    for set in sets {
        for i in 0..minimum_cube_amounts.len() {
            if minimum_cube_amounts[i] == 0 || minimum_cube_amounts[i] < set[i] {
                minimum_cube_amounts[i] = set[i];
            }
        }
    }

    minimum_cube_amounts
}
