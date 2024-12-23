use std::fs;
use std::io::Result;
use std::path::Path;
use std::env;

fn file_reader(file_path: &Path) -> Result<String> {
    let contents = fs::read_to_string(file_path)?;  // '?' operator propagates errors
    Ok(contents)
}

fn get_aoc_input(aoc_input_file: &str) -> Result<String> {
    let current_dir = env::current_dir()
        .expect("Failed to get current directory");
    let file_input_path = current_dir.join("inputs/").join(aoc_input_file);
    let contents = file_reader(file_input_path.as_path())?; 
    Ok(contents)
}

fn get_locations_id(input_file : String) -> (Vec<u32>, Vec<u32>) {
    let mut location_id_1: Vec<u32> = Vec::new();
    let mut location_id_2: Vec<u32> = Vec::new();
    for line in input_file.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match  parts[0].parse::<u32>() {
            Ok(integer) => location_id_1.push(integer),
            Err(e) => eprintln!("Parsing Error for location_id_1: {}", e),
        }
        match  parts[1].parse::<u32>() {
            Ok(integer) => location_id_2.push(integer),
            Err(e) => eprintln!("Parsing Error for location_id_2: {}", e),
        }
    }
    (location_id_1, location_id_2)
}

fn calculate_distances(location_id_1: &mut Vec<u32>, location_id_2: &mut Vec<u32>) -> u32 {
    location_id_1.sort();
    location_id_2.sort();
    let mut sum_distance: u32 = 0;
    for (id1, id2) in location_id_1.iter().zip(location_id_2.iter()) {
        let distance = id1.abs_diff(*id2);
        sum_distance += distance;
    }
    sum_distance
}

fn calculate_similarity(location_id_1: &mut Vec<u32>, location_id_2: &mut Vec<u32>) -> u32 {
    let mut sum_similiarity: u32 = 0;
    for id1 in location_id_1.iter() {
        let nb_in_location_id_2 = location_id_2.iter().filter(|x| **x == *id1).count() as u32;
        sum_similiarity += id1 * nb_in_location_id_2
    }
    sum_similiarity
}

fn main() {
    let contents = match get_aoc_input("input.txt") {  // 'match' handles the result of file_parser
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return;
        },
    };

    let (mut location_id_1, mut location_id_2) = get_locations_id(contents);
    let total_distance = calculate_distances(&mut location_id_1, &mut location_id_2);
    let total_similarity = calculate_similarity(&mut location_id_1, &mut location_id_2);
    println!("The total distance between lists is {}", total_distance);
    println!("The total similarity between lists is {}", total_similarity);
}
