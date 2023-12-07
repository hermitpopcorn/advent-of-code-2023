use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Race {
    pub time: usize,
    pub distance: usize,
}

pub fn parse_input_into_races(path: &str) -> Vec<Race> {
    let mut races = vec![];

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let split_first_line: Vec<&str> = first_line.split_whitespace().skip(1).collect();

    let second_line = lines.next().unwrap().unwrap();
    let split_second_line: Vec<&str> = second_line.split_whitespace().skip(1).collect();

    if split_first_line.len() != split_second_line.len() {
        panic!("Different data lengths");
    }

    for index in 0..split_first_line.len() {
        races.push(Race {
            time: split_first_line[index].parse().unwrap(),
            distance: split_second_line[index].parse().unwrap(),
        });
    }

    races
}

pub fn get_possible_win_scenarios_for_race(race: &Race) -> usize {
    let mut possible_win_scenarios = 0;

    for press_time in 1..race.time {
        let speed = press_time;
        let remaining_time = race.time - press_time;
        let distance_covered = speed * remaining_time;

        if distance_covered > race.distance {
            possible_win_scenarios += 1;
        }
    }

    possible_win_scenarios
}
