mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let file_load_error = "Could not load input data";

    println!("=== Day 01 ===");
    println!(
        "Total distance is: {}",
        day01::problem1::run(load_input_file("src/day01/input.txt").expect(file_load_error))
    );
    println!(
        "Similarity score is: {}",
        day01::problem2::run(load_input_file("src/day01/input.txt").expect(file_load_error))
    );

    println!("=== Day 02 ===");
    println!(
        "Number of safe reports: {}",
        day02::problem1::run(load_input_file("src/day02/input.txt").expect(file_load_error))
    );
    println!(
        "Number of safe reports after dampening: {}",
        day02::problem2::run(load_input_file("src/day02/input.txt").expect(file_load_error))
    );

    println!("=== Day 03 ===");
    println!(
        "Result of all multiplications: {}",
        day03::problem1::run(load_input_file("src/day03/input.txt").expect(file_load_error))
    );
    println!(
        "Result of all multiplications after adding 'logic': {}",
        day03::problem2::run(load_input_file("src/day03/input.txt").expect(file_load_error))
    );

    println!("=== Day 04 ===");
    println!(
        "XMAS count: {}",
        day04::problem1::run(load_input_file("src/day04/input.txt").expect(file_load_error))
    );
    println!(
        "MAS X count: {}",
        day04::problem2::run(load_input_file("src/day04/input.txt").expect(file_load_error))
    );

    println!("=== Day 05 ===");
    println!(
        "Middle page energy: {}",
        day05::problem1::run(load_input_file("src/day05/input.txt").expect(file_load_error))
    );
    println!(
        "Middle sorted page energy: {}",
        day05::problem2::run(load_input_file("src/day05/input.txt").expect(file_load_error))
    );

    println!("=== Day 06 ===");
    println!(
        "Districts covered: {}",
        day06::problem1::run(load_input_file("src/day06/input.txt").expect(file_load_error))
    );
    println!(
        "Potential patrol route loops after introducing a cheeky obstacle: {}",
        day06::problem2::run(load_input_file("src/day06/input.txt").expect(file_load_error))
    );

    println!("=== Day 07 ===");
    println!(
        "Total calibration result: {}",
        day07::problem1::run(load_input_file("src/day07/input.txt").expect(file_load_error))
    );
    println!(
        "Total calibration result: {}",
        day07::problem2::run(load_input_file("src/day07/input.txt").expect(file_load_error))
    );

    println!("=== Day 08 ===");
    println!(
        "Total anitnodes: {}",
        day08::problem1::run(load_input_file("src/day08/input.txt").expect(file_load_error))
    );
    println!(
        "Total anitnodes: {}",
        day08::problem2::run(load_input_file("src/day08/input.txt").expect(file_load_error))
    );
}

fn load_input_file(file_path: &str) -> Result<impl Iterator<Item = String>, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map_while(Result::ok))
}
