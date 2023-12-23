use crate::Diagram;

pub fn scan_for_almost_reflection_horizontally(diagram: &Diagram) -> Option<usize> {
    for line_index in 0..diagram.len() - 1 {
        if verify_almost_reflection_horizontally(diagram, line_index, line_index + 1) {
            return Some(line_index + 1);
        }
    }

    None
}

fn verify_almost_reflection_horizontally(
    diagram: &Diagram,
    mut line_back_index: usize,
    mut line_forward_index: usize,
) -> bool {
    let mut almost_matches = 0;

    let check_almost_matches = |a: &Vec<char>, b: &Vec<char>| -> bool {
        if a.len() != b.len() {
            panic!("a.len() != b.len()");
        }
        let len = a.len();
        let mut mismatches = 0;

        for index in 0..len {
            if a[index] != b[index] {
                mismatches += 1;
            }

            if mismatches > 1 {
                return false;
            }
        }

        if mismatches == 0 {
            return false;
        }

        true
    };

    loop {
        let line_back = &diagram[line_back_index];
        let line_forward = &diagram[line_forward_index];

        if *line_back != *line_forward {
            if check_almost_matches(line_back, line_forward) {
                almost_matches += 1;
            } else {
                return false;
            }
        }

        if almost_matches > 1 {
            return false;
        }

        if line_back_index == 0 || line_forward_index == diagram.len() - 1 {
            break;
        }

        line_back_index -= 1;
        line_forward_index += 1;
    }

    almost_matches == 1
}

pub fn scan_for_almost_reflection_vertically(diagram: &Diagram) -> Option<usize> {
    let translated_diagram = translate_diagram(diagram);

    scan_for_almost_reflection_horizontally(&translated_diagram)
}

fn translate_diagram(diagram: &Diagram) -> Diagram {
    let mut translated_diagram = vec![];

    let y_length = diagram.len();
    let x_length = diagram.first().unwrap().len();

    for x in 0..x_length {
        let mut line = vec![];
        for y in 0..y_length {
            line.push(diagram[y][x]);
        }
        translated_diagram.push(line);
    }

    translated_diagram
}
