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
    let masses: Vec<u32> = contents
        .split("\n")
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
    let input_filename = "input.txt";
    match get_module_masses_from_input(input_filename) {
        Ok(module_masses) => {
            let total_fuel: u32 = module_masses
                .into_iter()
                .map(|m| {
                    let module_mass = calculate_fuel_for_module(m);
                    let mut module_and_fuel_mass = module_mass;
                    let mut fuel_mass = module_mass;
                    while fuel_mass > 0 {
                        let added_mass = calculate_fuel_for_module(fuel_mass);
                        if added_mass > 0 {
                            module_and_fuel_mass += added_mass;
                        }
                        fuel_mass = added_mass;
                    }

                    return module_and_fuel_mass;
                })
                .sum();
            println!("total fuel {}", total_fuel);
        }
        Err(e) => println!(
            "Err encountered reading input file {}: {}",
            input_filename, e
        ),
    }
}
