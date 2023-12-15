use day12::{get_possible_arragements, parse_file_to_condition_records};

fn main() {
    let condition_records = parse_file_to_condition_records("input/real.txt");

    let mut total_possible_arrangements = 0;
    for condition_record in condition_records.iter() {
        let possible_arrangements = get_possible_arragements(condition_record);
        total_possible_arrangements += possible_arrangements;

        println!(
            "Notation: {}, Damaged grouping: {:?}, Possible arrangements: {}",
            condition_record.notation, condition_record.damaged_grouping, possible_arrangements
        )
    }

    println!(
        "Total possible arrangements: {}",
        total_possible_arrangements
    );
}
