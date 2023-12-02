fn main() {
    let inputs = std::fs::read_to_string("input/real.txt").unwrap();
    let inputs: Vec<&str> = inputs.split("\n").collect();

    let mut values = vec![];
    for input in inputs {
        if input.len() < 1 {
            continue;
        }

        let value = parse(input);
        match value {
            Some(value) => values.push(value),
            None => {}
        }

        println!("{} <- {}", if value.is_some() { "Y" } else { "N" }, input);
    }

    let answer: u32 = values.iter().sum();
    println!("Answer: {}", answer);
}

fn parse(input: &str) -> Option<u32> {
    let game = parse_game(input);

    let game_number = game.0;
    let game_sets = parse_game_sets_string_into_cube_arrays(&game.1);

    let valid = is_game_valid(game_sets);

    if valid {
        return Some(game_number);
    } else {
        return None;
    }
}

fn parse_game(input: &str) -> (u32, String) {
    let split: Vec<&str> = input.split(": ").collect();

    let game_number = parse_game_number(split.get(0).unwrap());
    let game_string = String::from(*split.get(1).unwrap());

    (game_number, game_string)
}

fn parse_game_number(input: &str) -> u32 {
    let without_first_five = &input[5..];
    let numeric_string = String::from(without_first_five);
    numeric_string.parse().unwrap()
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

fn is_game_valid(sets: Vec<[u32; 3]>) -> bool {
    for set in sets {
        if set[0] > 12 {
            return false;
        }
        if set[1] > 13 {
            return false;
        }
        if set[2] > 14 {
            return false;
        }
    }

    true
}
