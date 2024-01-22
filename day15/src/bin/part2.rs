use day15::{
    get_step_from_sequence, hash_sequence, parse_file_to_sequences, LensBox, Operation, Step,
};

fn main() {
    let sequences = parse_file_to_sequences("input/real.txt");

    let steps = sequences
        .into_iter()
        .map(|sequence| get_step_from_sequence(&sequence))
        .collect::<Vec<Step>>();

    let mut boxes: [LensBox; 256] = std::array::from_fn(|_| LensBox::new());
    for step in steps.iter() {
        let box_index = hash_sequence(&step.label);
        let the_box = &mut boxes[box_index];

        if step.operation == Operation::Dash {
            the_box.remove(&step.label);
        } else if step.operation == Operation::Equals {
            the_box.insert(step.label.clone(), step.focal_length.unwrap());
        }
    }

    let mut sum: usize = 0;
    for (box_number, lens_box) in boxes.iter().enumerate() {
        let base_strength = box_number + 1;
        for (index, focal_length) in lens_box.lenses.iter().enumerate() {
            let slot = index + 1;
            let focal_length = usize::from(*focal_length);
            let focusing_power = base_strength * slot * focal_length;
            sum += focusing_power;
        }
    }

    println!("{}", sum);
}
