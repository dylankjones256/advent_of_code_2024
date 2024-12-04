// Day 1 Solution - RUST
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::Reader;

fn parse_input_data(file_path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    
    let file = File::open(file_path)?; // opens the file at the specified path - ? will return any error thrown if file::open fails
    let mut reader = Reader::from_reader(file); // initialise CSV Reader object
    
    let mut list_one: Vec<u32> = vec![];
    let mut list_two: Vec<u32> = vec![];
    
    for result in reader.records() {
        let record = result?;
        let value_one: u32 = record[0].trim().parse()?; // the indexing here takes the first/second fields in the CSV
        let value_two: u32 = record[1].trim().parse()?;

        list_one.push(value_one);
        list_two.push(value_two);
    }
    
    Ok((list_one, list_two)) // this must be wrapped in an additional () because multi-value returns from functions must be Tuples
}

// file_path must be a borrowed string (&str) because borrowed types have known sizes which are mandatory for function parameters
fn calculate_distance(file_path: &str) -> Result<u64, Box<dyn Error>> {
    // compiler suggested adding the else here as we have no way to handle failures otherwise
    // we also needed to make the incoming Vectors mutable so sort_unstable() could be applied to them
    let Ok((mut list_one, mut list_two)) = parse_input_data(&file_path) else { todo!() };

    // Vectors store their data on the Heap
    // Jake pointed out that Vector sorting in Rust is not as fast as using other collection types e.g. HashMaps
    list_one.sort_unstable(); // this is faster and more memory efficient than sort()
    list_two.sort_unstable(); // this is faster and more memory efficient than sort()

    // use .copied() below so we get the actual value behind the &u32 reference provided by .iter()
    // we would have to do dereferencing otherwise with * I think?
    let total_distance: u64 = list_one
        .iter() // these are lazy like Transformations in Spark
        .copied()
        .zip(list_two.iter().copied())
        .map(|(a, b)| u64::abs_diff(a as u64, b as u64))
        .sum(); // this is like an Action in Spark - it retrieves the end result of your chained commands via summarisation/collection etc

    Ok(total_distance)
}

fn calculate_similarity_score(
    file_path: &str) // &str is essentially a view on the original file_path str
     -> Result<u32, Box<dyn Error>> { // returns a Result type of either n unsigned 32-bit integer or a Box with a dynamic error of any type
    
    let Ok((list_one, list_two)) = parse_input_data(&file_path) else { todo!() };

    let precalc_frequencies = list_two
        .into_iter() // create an iterator from a collection, consuming that collection in the process
        .fold( // 'folds' each element into an accumulator by applying an expression and returning a single value
            HashMap::new(), // initial value, a new and empty HashMap
            |mut map, value| { // closure that takes in our new map and the iterable variable in fold()
                // the HashMap here will look like this in the first iteration:
                // {12345: 0}
                // the HashMap will be extended dynamically as fold() iterates through our Vector
                // at the end, we have a KV store of each value and its count
                *map.entry(value).or_default() += 1; // we have to dereference map here to be able to modify it
                map
            }
    );
    
    let similarity_score: u32 = list_one
        .iter()
        .fold(
                0, // initial value (our cumulative score)
                |total, &value| { // use &value so we don't copy unnecessarily
                    total + value * precalc_frequencies // multiply our value by the occurrences and add it to the total
                        .get(&value) // fetch the count (value) for the current value (key) in the HashMap
                        .unwrap_or(&0) // handle where value is not a valid key in the HashMap (use 0 instead)
                }
    );

    Ok(similarity_score)
}

fn main() {
    let file_path: &str = "puzzle_input.csv";

    match calculate_distance(file_path) {
        Ok(total) => println!("Total distance: {:?}", total),
        Err(err) => eprintln!("Error: {:?}", err),
    };

    match calculate_similarity_score(file_path) {
        Ok(total) => println!("Similarity Score: {:?}", total),
        Err(err) => eprintln!("Error: {:?}", err),
    };
}