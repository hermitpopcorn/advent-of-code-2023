use std::collections::HashMap;

use day04::parse_card_into_card_number_and_matching_numbers;

fn main() {
    let inputs = std::fs::read_to_string("input/real.txt").unwrap();
    let inputs = inputs.split("\n").collect();

    let card_number_and_matching_numbers = parse_input_to_card_number_and_matching_numbers(inputs);

    let mut card_numbers_descending: Vec<u32> =
        card_number_and_matching_numbers.keys().cloned().collect();
    card_numbers_descending.sort_by(|a, b| b.cmp(a));

    let last_card_number = card_numbers_descending.first().unwrap().clone();
    let mut total_cards_won_by_card_number: HashMap<u32, u32> = HashMap::new();
    for card_number in card_numbers_descending {
        let matching_numbers_count = card_number_and_matching_numbers
            .get(&card_number)
            .unwrap()
            .len() as u32;

        let mut cards_won = 1;
        for won_card_number in (card_number + 1)..(card_number + matching_numbers_count + 1) {
            if won_card_number > last_card_number {
                break;
            }

            let sum = total_cards_won_by_card_number
                .get(&won_card_number)
                .unwrap();
            cards_won += sum;
        }

        total_cards_won_by_card_number.insert(card_number, cards_won);
    }

    let total_sum = total_cards_won_by_card_number
        .into_values()
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("Answer: {}", total_sum);
}

fn parse_input_to_card_number_and_matching_numbers(inputs: Vec<&str>) -> HashMap<u32, Vec<u32>> {
    let mut card_number_and_matching_numbers: HashMap<u32, Vec<u32>> = HashMap::new();
    for input in inputs {
        if input.len() < 1 {
            continue;
        }

        let (card_number, matching_numbers) =
            parse_card_into_card_number_and_matching_numbers(input);

        card_number_and_matching_numbers.insert(card_number, matching_numbers);
    }

    card_number_and_matching_numbers
}
