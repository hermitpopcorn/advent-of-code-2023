use day03::gearbox::parser::parse_gearbox;

fn main() {
    let input = std::fs::read_to_string("input/real.txt").unwrap();

    let gearbox = parse_gearbox(&input);
    let gears = gearbox.get_gears();

    let mut answer: usize = 0;
    for gear in gears {
        let gear_ratio = gear[0].value * gear[1].value;
        answer += gear_ratio;
    }
    println!("Answer: {}", answer);
}
