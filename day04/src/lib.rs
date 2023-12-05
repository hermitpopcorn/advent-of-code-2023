use std::collections::HashSet;

pub fn parse_card_into_matching_numbers(input: &str) -> Vec<u32> {
    let (_card_number, card_body) = get_card_number_and_body(input);

    let (winning_numbers, my_numbers) = parse_card_body_into_number_vectors(&card_body);

    get_matching_numbers(winning_numbers, my_numbers)
}

fn get_card_number_and_body(input: &str) -> (u32, String) {
    let split: Vec<&str> = input.split(": ").collect();

    let card_number = parse_card_number(split.get(0).unwrap());
    let card_body = String::from(*split.get(1).unwrap());

    (card_number, card_body)
}

fn parse_card_number(input: &str) -> u32 {
    let without_first_five = &input[5..];
    let numeric_string = String::from(without_first_five);
    numeric_string.trim().parse().unwrap()
}

fn parse_card_body_into_number_vectors(input: &str) -> (Vec<u32>, Vec<u32>) {
    let split_body: Vec<&str> = input.split(" | ").collect();

    let winning_numbers = parse_string_of_numbers_into_number_vector(split_body.get(0).unwrap());
    let my_numbers = parse_string_of_numbers_into_number_vector(split_body.get(1).unwrap());

    (winning_numbers, my_numbers)
}

fn parse_string_of_numbers_into_number_vector(input: &str) -> Vec<u32> {
    let split: Vec<&str> = input.split(" ").collect();

    let mut numbers = vec![];
    for number in split {
        if number.len() < 1 {
            continue;
        }

        let numeric = number.parse().unwrap();
        numbers.push(numeric);
    }

    numbers
}

fn get_matching_numbers(winning_numbers: Vec<u32>, my_numbers: Vec<u32>) -> Vec<u32> {
    let set: HashSet<u32> = winning_numbers.into_iter().collect();
    my_numbers.into_iter().filter(|n| set.contains(n)).collect()
}
