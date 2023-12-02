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

    let mut input_char = |the_char: char| {
        if first.is_none() {
            first = Some(the_char);
        } else {
            second = Some(the_char);
        }
    };

    for character in input.chars() {
        if character.is_numeric() {
            input_char(character);
        }
    }

    if second.is_none() {
        second = first.clone();
    }

    (first.unwrap(), second.unwrap())
}

fn merge_chars_into_u32(chars: (char, char)) -> u32 {
    let numeric_string = format!("{}{}", chars.0, chars.1);
    numeric_string.parse().unwrap()
}
