use day13::{
    calculate_scores, parse_file_to_diagrams, scan_for_reflection_horizontally,
    scan_for_reflection_vertically, Diagram, Score,
};

fn main() {
    let diagrams = parse_file_to_diagrams("input/example.txt");

    let mut scores = vec![];

    for (index, unsmudged_diagram) in diagrams.into_iter().enumerate() {
        let horizontal_reflection = scan_for_reflection_horizontally(&unsmudged_diagram);
        let vertical_reflection = scan_for_reflection_vertically(&unsmudged_diagram);

        let original;
        if let Some(horizontal_reflection) = horizontal_reflection {
            original = Score::Horizontal(horizontal_reflection);
        } else if let Some(vertical_reflection) = vertical_reflection {
            original = Score::Vertical(vertical_reflection);
        } else {
            panic!("No original found");
        }

        let smudge_dupe_diagrams = create_smudge_dupes(unsmudged_diagram);

        for diagram in smudge_dupe_diagrams {
            let horizontal_reflection = scan_for_reflection_horizontally(&diagram.diagram);
            let vertical_reflection = scan_for_reflection_vertically(&diagram.diagram);

            if horizontal_reflection.is_none() && vertical_reflection.is_none() {
                continue;
            }

            let score;

            if let Some(horizontal_reflection) = horizontal_reflection {
                score = Score::Horizontal(horizontal_reflection);
            } else if let Some(vertical_reflection) = vertical_reflection {
                score = Score::Vertical(vertical_reflection);
            } else {
                panic!("Undefined behavior");
            }

            if score == original {
                continue;
            }

            println!("index: {}", index);
            println!("smudge location: {:?}", diagram.smudge_location);
            println!("score: {:?}", score);

            println!("smudge diagram:\n{}", draw_diagram(&diagram.diagram));

            scores.push(score);

            break;
        }
    }

    let score = calculate_scores(scores);
    println!("Score: {}", score);
}

struct SmudgeDiagram {
    diagram: Diagram,
    smudge_location: (usize, usize),
}

fn create_smudge_dupes(diagram: Diagram) -> Vec<SmudgeDiagram> {
    let mut smudge_dupes = vec![];

    let y_length = diagram.len();
    let x_length = diagram.first().unwrap().len();

    let switch_character = |character: char| -> char {
        match character {
            '.' => '#',
            '#' => '.',
            _ => panic!("Unexpected character: {}", character),
        }
    };

    for y in 0..y_length {
        for x in 0..x_length {
            let mut smudge_dupe = diagram.clone();
            smudge_dupe[y][x] = switch_character(smudge_dupe[y][x]);
            smudge_dupes.push(SmudgeDiagram {
                diagram: smudge_dupe,
                smudge_location: (x, y),
            });
        }
    }

    smudge_dupes
}

fn draw_diagram(diagram: &Diagram) -> String {
    let mut output = String::new();

    for line in diagram {
        for character in line {
            output.push(*character);
        }
        output.push('\n');
    }

    output
}
