use day13::{
    almost::{scan_for_almost_reflection_horizontally, scan_for_almost_reflection_vertically},
    calculate_scores, parse_file_to_diagrams, Score,
};

fn main() {
    let diagrams = parse_file_to_diagrams("input/real.txt");

    let mut scores = vec![];

    for diagram in diagrams {
        let horizontal_reflection = scan_for_almost_reflection_horizontally(&diagram);
        let vertical_reflection = scan_for_almost_reflection_vertically(&diagram);

        if let Some(horizontal_reflection) = horizontal_reflection {
            scores.push(Score::Horizontal(horizontal_reflection));
        }
        if let Some(vertical_reflection) = vertical_reflection {
            scores.push(Score::Vertical(vertical_reflection));
        }
    }

    let score = calculate_scores(scores);
    println!("Score: {}", score);
}
