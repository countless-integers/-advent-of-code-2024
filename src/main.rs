mod day01;
mod day02;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let file_load_error = "Could not load input data";

    println!("=== Day 01 ===");
    println!(
        "Total distance is: {}",
        day01::problem1::run(load_input_file("src/day01/input.txt").expect(&file_load_error))
    );
    println!(
        "Similarity score is: {}",
        day01::problem2::run(load_input_file("src/day01/input.txt").expect(&file_load_error))
    );

    println!("=== Day 02 ===");
    println!(
        "Number of safe reports: {}",
        day02::problem1::run(load_input_file("src/day02/input.txt").expect(&file_load_error))
    );
    println!(
        "Number of safe reports after dampening: {}",
        day02::problem2::run(load_input_file("src/day02/input.txt").expect(&file_load_error))
    );
}

fn load_input_file(file_path: &str) -> Result<impl Iterator<Item = String>, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().filter_map(|line| line.ok()))
}
