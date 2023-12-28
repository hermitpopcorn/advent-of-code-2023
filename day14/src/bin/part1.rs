use day14::{calculate_load, parse_file_to_lines, tilt_line};

fn main() {
    let lines = parse_file_to_lines("input/real.txt");

    let mut loads = vec![];
    for line in lines {
        let tilted_line = tilt_line(line);
        let load = calculate_load(tilted_line);
        loads.push(load);
    }

    let sum = loads.iter().sum::<usize>();
    println!("Sum: {}", sum);
}
