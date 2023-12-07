use day04::parse_card_into_card_number_and_matching_numbers;

fn main() {
    let inputs = std::fs::read_to_string("input/real.txt").unwrap();
    let inputs: Vec<&str> = inputs.split("\n").collect();

    let mut values = vec![];
    for input in inputs {
        if input.len() < 1 {
            continue;
        }

        let matching_numbers = parse_card_into_card_number_and_matching_numbers(input).1;
        let points = calculate_points_from_matching_numbers(matching_numbers);
        values.push(points);

        println!("{} <- {}", points, input);
    }

    let answer: u32 = values.iter().sum();
    println!("Answer: {}", answer);
}

fn calculate_points_from_matching_numbers(matching_numbers: Vec<u32>) -> u32 {
    if matching_numbers.len() < 1 {
        return 0;
    }

    let base_points: u32 = 2;
    let power: u32 = (matching_numbers.len() - 1) as u32;

    base_points.pow(power)
}
