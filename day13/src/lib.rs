use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub type Diagram = Vec<Vec<char>>;

pub fn parse_file_to_diagrams(path: &str) -> Vec<Diagram> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut diagrams = vec![];

    let mut diagram = vec![];
    for lines in lines {
        let line = lines.unwrap();
        if line.len() == 0 {
            diagrams.push(diagram.clone());
            diagram.clear();
            continue;
        }

        let diagram_line = line.chars().collect::<Vec<char>>();
        diagram.push(diagram_line);
    }

    if diagram.len() > 0 {
        diagrams.push(diagram);
    }

    diagrams
}

pub fn scan_for_reflection_horizontally(diagram: &Diagram) -> Option<usize> {
    for line_index in 0..diagram.len() - 1 {
        if verify_reflection_horizontally(diagram, line_index, line_index + 1) {
            return Some(line_index + 1);
        }
    }

    None
}

fn verify_reflection_horizontally(
    diagram: &Diagram,
    mut line_back_index: usize,
    mut line_forward_index: usize,
) -> bool {
    loop {
        let line_back = &diagram[line_back_index];
        let line_forward = &diagram[line_forward_index];

        if *line_back != *line_forward {
            return false;
        }

        if line_back_index == 0 || line_forward_index == diagram.len() - 1 {
            break;
        }

        line_back_index -= 1;
        line_forward_index += 1;
    }

    true
}

pub fn scan_for_reflection_vertically(diagram: &Diagram) -> Option<usize> {
    let translated_diagram = translate_diagram(diagram);

    scan_for_reflection_horizontally(&translated_diagram)
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

pub enum Score {
    Horizontal(usize),
    Vertical(usize),
}

pub fn calculate_scores(scores: Vec<Score>) -> usize {
    let mut tally = 0;
    for score in scores {
        match score {
            Score::Horizontal(number) => {
                tally += number * 100;
            }
            Score::Vertical(number) => {
                tally += number;
            }
        }
    }

    tally
}
