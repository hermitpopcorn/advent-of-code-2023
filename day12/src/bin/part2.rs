use day12::{get_possible_arragements, parse_file_to_condition_records, ConditionRecord};
use rayon::prelude::*;

const UNFOLD_REPEATS: usize = 5;

fn main() {
    let condition_records = parse_file_to_condition_records("input/real.txt");

    let condition_records = condition_records
        .iter()
        .map(|cr| unfold_condition_record(cr))
        .collect::<Vec<ConditionRecord>>();

    let possible_arrangements = condition_records.par_iter().map(|condition_record| {
        let possible_arrangements = get_possible_arragements(condition_record);

        println!(
            "Notation: {}, Damaged grouping: {:?}, Possible arrangements: {}",
            condition_record.notation, condition_record.damaged_grouping, possible_arrangements
        );

        possible_arrangements
    });

    let total_possible_arrangements = possible_arrangements.sum::<usize>();
    println!(
        "Total possible arrangements: {}",
        total_possible_arrangements
    );
}

fn unfold_condition_record(condition_record: &ConditionRecord) -> ConditionRecord {
    let mut unfolded_notation_vector = vec![];
    for _ in 0..UNFOLD_REPEATS {
        unfolded_notation_vector.push(condition_record.notation.clone());
    }
    let unfolded_notation = unfolded_notation_vector.join("?");

    let unfolded_damaged_grouping = condition_record.damaged_grouping.repeat(UNFOLD_REPEATS);

    ConditionRecord {
        notation: unfolded_notation,
        damaged_grouping: unfolded_damaged_grouping,
    }
}
