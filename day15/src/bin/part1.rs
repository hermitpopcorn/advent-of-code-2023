use day15::{hash_sequence, parse_file_to_sequences};

fn main() {
    let sequences = parse_file_to_sequences("input/real.txt");

    let hashes = sequences
        .into_iter()
        .map(|sequence| hash_sequence(&sequence))
        .collect::<Vec<usize>>();

    let sum = hashes.into_iter().reduce(|acc, hash| acc + hash).unwrap();
    println!("{}", sum);
}
