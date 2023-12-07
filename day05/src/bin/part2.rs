use day05::{
    get_location_from_seed, parse_input_into_raw_data, parse_raw_data_into_almanac, Almanac,
};
use rayon::prelude::*;

fn main() {
    let raw_data = parse_input_into_raw_data("input/real.txt");
    let almanac: Almanac = parse_raw_data_into_almanac(raw_data);

    let seed_ranges = parse_seeds_as_seed_ranges(&almanac.seeds);

    let mut smallest_location = None;
    for seed_range in seed_ranges {
        let locations: Vec<usize> = (seed_range.0..seed_range.0 + seed_range.1)
            .into_par_iter()
            .map(|seed| get_location_from_seed(seed, &almanac.maps))
            .collect();

        let location = locations.into_iter().min().unwrap();

        if smallest_location.is_none() {
            smallest_location = Some(location);
        } else {
            if smallest_location.unwrap() > location {
                smallest_location = Some(location);
            }
        }
    }

    let answer = smallest_location.unwrap();
    println!("Answer: {}", answer);
}

fn parse_seeds_as_seed_ranges(seeds: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut seed_ranges = vec![];

    for index in 0..seeds.len() / 2 {
        let start = seeds[index * 2];
        let length = seeds[index * 2 + 1];

        seed_ranges.push((start, length));
    }

    seed_ranges
}
