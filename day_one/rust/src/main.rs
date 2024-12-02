// Day 1 Solution - RUST

use std::error::Error;
use std::fs::File;
use csv::Reader;

// file_path must be a borrowed string (&str) because borrowed types have known sizes which are mandatory for function parameters
fn calculate_distance(file_path: &str) -> Result<u64, Box<dyn Error>> {

    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    
    let mut list_one: Vec<u32> = vec![];
    let mut list_two: Vec<u32> = vec![];
    
    for result in reader.records() {
        let record = result?;
        let value_one: u32 = record[0].trim().parse()?;
        let value_two: u32 = record[1].trim().parse()?;

        list_one.push(value_one);
        list_two.push(value_two);
    }

    list_one.sort_unstable(); // this is faster and more memory efficient than sort()
    list_two.sort_unstable(); // this is faster and more memory efficient than sort()

    // use .copied() below so we get the actual value behind the &u32 reference provided by .iter()
    // we would have to do dereferencing otherwise with * I think
    let total_distance: u64 = list_one
        .iter()
        .copied()
        .zip(list_two.iter().copied())
        .map(|(a, b)| u64::abs_diff(a as u64, b as u64))
        .sum();

    Ok(total_distance)
}

fn main() {
    let file_path = "puzzle_input.csv";
    match calculate_distance(file_path) {
        Ok(total) => println!("Total distance: {:?}", total),
        Err(err) => eprintln!("Error: {:?}", err),
    };
}