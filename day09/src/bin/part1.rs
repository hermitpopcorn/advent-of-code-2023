use day09::{
    calculate_next_history_upwards, get_difference_matrix, parse_file_into_vector_of_histories,
    History, HistoryValue,
};

fn main() {
    let histories = parse_file_into_vector_of_histories("input/real.txt");

    let mut next_history_values = vec![];

    for history in histories {
        let difference_matrix = get_difference_matrix(history);

        let difference_matrix_with_next_value = calculate_next_history_upwards(difference_matrix);

        let next_history_value = get_next_history_value(difference_matrix_with_next_value);

        next_history_values.push(next_history_value);
    }

    let answer = next_history_values.iter().sum::<HistoryValue>();
    println!("Answer: {}", answer);
}

fn get_next_history_value(difference_matrix: Vec<History>) -> HistoryValue {
    *difference_matrix[0].last().unwrap()
}
