use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub type HistoryValue = isize;
pub type History = Vec<HistoryValue>;

pub fn parse_file_into_vector_of_histories(path: &str) -> Vec<History> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut histories = vec![];

    for lines in lines {
        let line = lines.unwrap();

        let numbers: Vec<HistoryValue> = line
            .split_whitespace()
            .map(|number| number.parse::<HistoryValue>().unwrap())
            .collect();

        histories.push(numbers);
    }

    histories
}

pub fn get_difference_matrix(history: History) -> Vec<History> {
    let starting_matrix = vec![history.clone()];
    let difference_matrix = get_difference_matrix_recursive(starting_matrix, history);

    let difference_matrix = calculate_next_history_upwards(difference_matrix);

    difference_matrix
}

fn get_difference_matrix_recursive(mut matrix: Vec<History>, history: History) -> Vec<History> {
    let mut difference_row = vec![];
    for number in 0..history.len() - 1 {
        difference_row.push(history[number + 1] - history[number]);
    }

    matrix.push(difference_row.clone());

    if !is_final_difference_row(&difference_row) {
        return get_difference_matrix_recursive(matrix, difference_row);
    }

    matrix
}

fn is_final_difference_row(difference_row: &History) -> bool {
    difference_row.iter().all(|number| *number == 0)
}

fn calculate_next_history_upwards(mut matrix: Vec<History>) -> Vec<History> {
    for row_index in (0..matrix.len()).rev() {
        if row_index == matrix.len() - 1 {
            matrix[row_index].push(0);
            continue;
        }

        let end_of_last_row = matrix[row_index + 1].last().unwrap();
        let end_of_this_row = matrix[row_index].last().unwrap();
        let next_history_number = end_of_last_row + end_of_this_row;

        matrix[row_index].push(next_history_number);
    }

    matrix
}
