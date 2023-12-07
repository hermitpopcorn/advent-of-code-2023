use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: [char; 5],
    pub bid: usize,
}

pub fn parse_input_into_hands(path: &str) -> Vec<Hand> {
    let mut hands = vec![];

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let split: Vec<&str> = line.split_whitespace().collect();
        let cards: Vec<char> = split[0].chars().collect();

        hands.push(Hand {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            bid: split[1].parse().unwrap(),
        })
    }

    hands
}

pub fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    let winner = get_winner_by_comparing_hands(&hand1.cards, &hand2.cards);
    if winner.is_some() {
        let winner = winner.unwrap();
        if winner == &hand1.cards {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    for i in 0..5 {
        let card1 = &hand1.cards[i];
        let card2 = &hand2.cards[i];

        let winner = get_winner_by_comparing_cards(card1, card2);
        if winner.is_some() {
            let winner = winner.unwrap();
            if winner == card1 {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
    }

    Ordering::Equal
}

fn get_winner_by_comparing_hands<'a>(
    hand1: &'a [char; 5],
    hand2: &'a [char; 5],
) -> Option<&'a [char; 5]> {
    let hand1_strength = get_hand_type(hand1) as u8;
    let hand2_strength = get_hand_type(hand2) as u8;

    if hand1_strength > hand2_strength {
        Some(&hand1)
    } else if hand1_strength < hand2_strength {
        Some(&hand2)
    } else {
        None
    }
}

fn get_hand_type(hand: &[char; 5]) -> HandType {
    let mut card_counts: HashMap<char, u8> = HashMap::new();

    for card in hand {
        let count = card_counts.entry(*card).or_insert(0);
        *count += 1;
    }

    let card_types = card_counts.keys();

    if card_types.len() == 1 {
        return HandType::FiveOfAKind;
    }

    if card_types.len() == 2 {
        let sorted_counts = get_sorted_card_counts(&card_counts);

        if sorted_counts[0] == 4 {
            return HandType::FourOfAKind;
        } else if sorted_counts[0] == 3 {
            return HandType::FullHouse;
        } else {
            panic!("Unknown hand type")
        }
    }

    if card_types.len() == 3 {
        let sorted_counts = get_sorted_card_counts(&card_counts);

        if sorted_counts[0] == 3 {
            return HandType::ThreeOfAKind;
        } else if sorted_counts[0] == 2 && sorted_counts[1] == 2 {
            return HandType::TwoPair;
        } else {
            panic!("Unknown hand type")
        }
    }

    if card_types.len() == 4 {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn get_sorted_card_counts(card_counts: &HashMap<char, u8>) -> Vec<u8> {
    let mut card_count_values: Vec<u8> = card_counts.clone().into_values().collect();
    card_count_values.sort_by(|a, b| b.cmp(a));
    card_count_values
}

fn get_winner_by_comparing_cards<'a>(card1: &'a char, card2: &'a char) -> Option<&'a char> {
    let card1_strength = get_card_strength(card1);
    let card2_strength = get_card_strength(card2);

    if card1_strength > card2_strength {
        Some(&card1)
    } else if card1_strength < card2_strength {
        Some(&card2)
    } else {
        None
    }
}

fn get_card_strength(card: &char) -> u8 {
    match *card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card"),
    }
}
