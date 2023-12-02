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
    let chars = get_first_and_last_numeric_chars(input);

    merge_chars_into_u32(chars)
}

fn get_first_and_last_numeric_chars(input: &str) -> (char, char) {
    let mut first: Option<char> = None;
    let mut second: Option<char> = None;

    let mut unmatched_this_far = String::from("");

    // First pass (start -> end)
    for character in input.chars() {
        if character.is_numeric() {
            first = Some(character);
            break;
        }

        unmatched_this_far.push(character);
        if unmatched_this_far.len() < 3 {
            continue;
        }

        let has_numeric_string = get_matching_numeric_string_into_char(&unmatched_this_far);
        match has_numeric_string {
            Some(character) => {
                first = Some(character);
                break;
            }
            None => continue,
        }
    }

    // Second pass (start <- end)
    unmatched_this_far.clear();
    for character in input.chars().rev() {
        if character.is_numeric() {
            second = Some(character);
            break;
        }

        unmatched_this_far.insert(0, character);
        if unmatched_this_far.len() < 3 {
            continue;
        }

        let has_numeric_string = get_matching_numeric_string_into_char(&unmatched_this_far);
        match has_numeric_string {
            Some(character) => {
                second = Some(character);
                break;
            }
            None => continue,
        }
    }

    if second.is_none() {
        second = first.clone();
    }

    (first.unwrap(), second.unwrap())
}

fn get_matching_numeric_string_into_char(input: &str) -> Option<char> {
    let numeric_strings = std::collections::HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for (possible_match, numeric_char) in numeric_strings {
        if input.contains(possible_match) {
            return Some(numeric_char);
        }
    }

    None
}

fn merge_chars_into_u32(chars: (char, char)) -> u32 {
    let numeric_string = format!("{}{}", chars.0, chars.1);
    numeric_string.parse().unwrap()
}
