use day17::{HeatLossMap, Pathfinder};

fn main() {
    let heat_loss_map = HeatLossMap::from_file_path("input/real.txt");
    let mut pathfinder = Pathfinder::create_from_heat_loss_map(&heat_loss_map);
    pathfinder.calculate_heat_losses();
}
