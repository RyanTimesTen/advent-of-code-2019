use math::round;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_module_masses_from_input(filename: &str) -> std::io::Result<Vec<u32>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let masses_str: Vec<&str> = contents.split("\n").collect();
    let masses: Vec<u32> = masses_str
        .into_iter()
        .map(|m| m.parse::<u32>().unwrap())
        .collect();
    Ok(masses)
}

fn calculate_fuel_for_module(module_mass: u32) -> u32 {
    let final_subtraction_amount = 2;
    let mass_divided_and_floored = round::floor((module_mass / 3) as f64, 0) as u32;
    let safeguared_mass = cmp::max(mass_divided_and_floored, final_subtraction_amount);
    return safeguared_mass - final_subtraction_amount;
}

fn main() {
    let mut total_fuel: u32 = 0;

    let input_filename = "input.txt";
    match get_module_masses_from_input(input_filename) {
        Ok(module_masses) => {
            total_fuel = module_masses
                .into_iter()
                .map(|m| calculate_fuel_for_module(m))
                .sum()
        }
        Err(e) => println!(
            "Err encountered reading input file {}: {}",
            input_filename, e
        ),
    }

    println!("total fuel {}", total_fuel);
}
