use day05::{get_location_from_seed, parse_input_into_raw_data, parse_raw_data_into_almanac};

fn main() {
    let raw_data = parse_input_into_raw_data("input/real.txt");
    let almanac = parse_raw_data_into_almanac(raw_data);

    let mut locations = vec![];
    for seed in almanac.seeds {
        let location = get_location_from_seed(seed, &almanac.maps);
        locations.push(location);

        println!("Seed: {}, Location: {}", seed, location);
    }

    let answer = *locations.iter().min().unwrap();
    println!("Answer: {}", answer);
}
