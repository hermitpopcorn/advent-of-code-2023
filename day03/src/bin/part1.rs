use day03::gearbox::parser::parse_gearbox;

fn main() {
    let input = std::fs::read_to_string("input/real.txt").unwrap();

    let gearbox = parse_gearbox(&input);
    let part_numbers = gearbox.get_part_numbers();

    let mut answer: usize = 0;
    for part_number in part_numbers {
        answer += part_number.value;
    }
    println!("Answer: {}", answer);
}
