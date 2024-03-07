use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct HeatLossMap {
    pub matrix: Vec<Vec<u8>>,
    horizontal_length: usize,
    vertical_length: usize,
}

impl HeatLossMap {
    pub fn from_file_path(path: &str) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut heat_loss_matrix = vec![];
        let mut horizontal_length = 0;

        for line in lines {
            let line = line.unwrap();

            if line.is_empty() {
                break;
            }

            let mut heat_loss_vector = vec![];

            for index in 0..line.len() {
                let line_str = line.as_str();
                let char_str = &line_str[index..index + 1];
                let heat_loss: u8 = u8::from_str_radix(char_str, 10).unwrap();
                heat_loss_vector.push(heat_loss);
            }

            horizontal_length = heat_loss_vector.len();
            heat_loss_matrix.push(heat_loss_vector);
        }

        let vertical_length = heat_loss_matrix.len();

        Self {
            matrix: heat_loss_matrix,
            horizontal_length,
            vertical_length,
        }
    }
}

pub type Coordinates = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Pathfinder<'a> {
    map: &'a HeatLossMap,
    start_coordinates: Coordinates,
    end_coordinates: Coordinates,
    lowest_heat_loss_at_finish: f64,
}

impl<'a> Pathfinder<'a> {
    pub fn create_from_heat_loss_map(map: &'a HeatLossMap) -> Pathfinder<'a> {
        let start_coordinates = (0, 0);
        let end_coordinates = (map.horizontal_length - 1, map.vertical_length - 1);
        let lowest_heat_loss_at_finish = f64::INFINITY;

        Self {
            map,
            start_coordinates,
            end_coordinates,
            lowest_heat_loss_at_finish,
        }
    }

    fn create_infinity_heat_loss_values_from_matrix(&self) -> Vec<Vec<f64>> {
        let mut heat_loss_values = vec![];
        for y in 0..self.map.vertical_length {
            let mut heat_loss_values_line = vec![];
            for x in 0..self.map.horizontal_length {
                if x == self.start_coordinates.0 && y == self.start_coordinates.1 {
                    heat_loss_values_line.push(0 as f64);
                    continue;
                }

                heat_loss_values_line.push(f64::INFINITY);
            }
            heat_loss_values.push(heat_loss_values_line);
        }
        heat_loss_values
    }

    pub fn calculate_heat_losses(&mut self) {
        self.lowest_heat_loss_at_finish = f64::INFINITY;

        let initial_direction_history = VecDeque::from([]);
        let initial_heat_loss_total_matrix = self.create_infinity_heat_loss_values_from_matrix();

        self.step(
            0.0,
            initial_heat_loss_total_matrix,
            initial_direction_history,
            self.start_coordinates,
        );

        println!("{:#?}", self.lowest_heat_loss_at_finish);
    }

    fn step(
        &mut self,
        carryover_heat_loss_total: f64,
        mut heat_loss_total_matrix: Vec<Vec<f64>>,
        mut direction_history: VecDeque<Direction>,
        current_coordinates: Coordinates,
    ) {
        if carryover_heat_loss_total >= self.lowest_heat_loss_at_finish {
            return;
        }
        if current_coordinates == self.end_coordinates {
            self.lowest_heat_loss_at_finish = carryover_heat_loss_total;
            println!("New record: {}", carryover_heat_loss_total);
            return;
        }

        while direction_history.len() > 3 {
            direction_history.pop_front();
        }

        let adjacent_paths = self.get_adjacent_paths(
            carryover_heat_loss_total,
            &mut heat_loss_total_matrix,
            &direction_history,
            &current_coordinates,
        );

        for adjacent_path in adjacent_paths {
            let heat_loss = adjacent_path.2;
            let coordinates = adjacent_path.1;

            let direction = adjacent_path.0;
            let mut direction_history = direction_history.clone();
            direction_history.push_back(direction);

            self.step(
                heat_loss,
                heat_loss_total_matrix.clone(),
                direction_history,
                coordinates,
            );
        }
    }

    fn get_adjacent_paths(
        &self,
        carryover_heat_loss_total: f64,
        mut heat_loss_total_matrix: &mut Vec<Vec<f64>>,
        direction_history: &VecDeque<Direction>,
        coordinates: &Coordinates,
    ) -> Vec<(Direction, Coordinates, f64)> {
        let mut adjacent_heat_losses = vec![];
        let adjacent_coordinates = self.get_adjacent_coordinates(&coordinates, &direction_history);

        for adjacent in adjacent_coordinates {
            let dir = adjacent.0;
            let c = adjacent.1;

            let heat_loss = Self::get_heat_loss_total_at(&heat_loss_total_matrix, &c);
            if heat_loss != f64::INFINITY {
                continue;
            }

            let additional_heat_loss = self.get_heat_loss_amount_at(&c) as f64;
            let calculated_heat_loss = carryover_heat_loss_total + additional_heat_loss;
            Self::store_heat_loss_total_at(&mut heat_loss_total_matrix, &c, calculated_heat_loss);
            adjacent_heat_losses.push((dir, c, calculated_heat_loss));
        }

        adjacent_heat_losses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        adjacent_heat_losses
    }

    fn get_adjacent_coordinates(
        &self,
        coordinates: &Coordinates,
        taken_directions: &VecDeque<Direction>,
    ) -> Vec<(Direction, Coordinates)> {
        let mut adjacent_coordinates = vec![];
        let jumps: [(Direction, (isize, isize)); 4] = [
            (Direction::Down, (0, 1)),
            (Direction::Right, (1, 0)),
            (Direction::Up, (0, -1)),
            (Direction::Left, (-1, 0)),
        ];

        println!("{:?}", taken_directions);

        let last_direction = taken_directions.back();

        let mut direction_taken_three_times = None;
        if taken_directions.len() >= 3 {
            let last_three_directions = taken_directions
                .clone()
                .into_iter()
                .rev()
                .take(3)
                .collect::<VecDeque<Direction>>();

            let mut prev_direction = None;
            let mut consecutive = 0;
            last_three_directions.iter().for_each(|direction| {
                if prev_direction.is_none() {
                    prev_direction = Some(direction);
                    consecutive = 1;
                    return;
                }

                let prev_direction = prev_direction.unwrap();
                if prev_direction == direction {
                    consecutive += 1;
                } else {
                    consecutive = 1;
                }
            });

            if consecutive >= 3 {
                direction_taken_three_times = Some(prev_direction.unwrap().clone());
            }
        }

        for jump in jumps {
            let jump_direction = jump.0;

            // No going backwards
            if last_direction.is_some() {
                let last_direction = *last_direction.unwrap();
                if last_direction == Direction::Down && jump_direction == Direction::Up {
                    continue;
                }
                if last_direction == Direction::Up && jump_direction == Direction::Down {
                    continue;
                }
                if last_direction == Direction::Left && jump_direction == Direction::Right {
                    continue;
                }
                if last_direction == Direction::Right && jump_direction == Direction::Left {
                    continue;
                }
            }

            // No 4 consecutive directions
            if direction_taken_three_times.is_some() {
                let direction_taken_three_times = direction_taken_three_times.unwrap();
                if direction_taken_three_times == jump_direction {
                    continue;
                }
            }

            let jump_x = coordinates.0.checked_add_signed(jump.1 .0);
            let jump_y = coordinates.1.checked_add_signed(jump.1 .1);
            if jump_x.is_none() || jump_y.is_none() {
                continue;
            }

            let jump_x = jump_x.unwrap();
            let jump_y = jump_y.unwrap();
            if jump_x >= self.map.horizontal_length || jump_y >= self.map.vertical_length {
                continue;
            }

            adjacent_coordinates.push((jump_direction, (jump_x, jump_y)));
        }

        adjacent_coordinates
    }

    fn get_heat_loss_amount_at(&self, coordinates: &Coordinates) -> u8 {
        self.map
            .matrix
            .get(coordinates.1)
            .unwrap()
            .get(coordinates.0)
            .unwrap()
            .clone()
    }

    fn get_heat_loss_total_at(
        heat_loss_total_matrix: &Vec<Vec<f64>>,
        coordinates: &Coordinates,
    ) -> f64 {
        heat_loss_total_matrix
            .get(coordinates.1)
            .unwrap()
            .get(coordinates.0)
            .unwrap()
            .clone()
    }

    fn store_heat_loss_total_at(
        heat_loss_total_matrix: &mut Vec<Vec<f64>>,
        coordinates: &Coordinates,
        total: f64,
    ) {
        let value = heat_loss_total_matrix
            .get_mut(coordinates.1)
            .unwrap()
            .get_mut(coordinates.0)
            .unwrap();

        *value = total;
    }
}
