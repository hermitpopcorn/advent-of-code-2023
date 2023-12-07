use day07::{compare_hands, parse_input_into_hands};

fn main() {
    let mut hands = parse_input_into_hands("input/real.txt");

    hands.sort_by(|a, b| compare_hands(a, b, true));

    let mut total_bid = 0;
    for index in 0..hands.len() {
        let hand = &hands[index];
        let rank = index + 1;

        total_bid += hand.bid * rank;
    }

    println!("Answer: {}", total_bid);
}
