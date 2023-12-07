use day06::{get_possible_win_scenarios_for_race, parse_input_into_races};

fn main() {
    let races = parse_input_into_races("input/real1.txt");

    let mut possible_win_scenarios_for_each_race = vec![];
    for race in races {
        let possible_win_scenarios = get_possible_win_scenarios_for_race(&race);

        possible_win_scenarios_for_each_race.push(possible_win_scenarios);
    }

    let answer = possible_win_scenarios_for_each_race
        .iter()
        .fold(1, |accumulator, &value| accumulator * value);
    println!("{:#?}", answer);
}
