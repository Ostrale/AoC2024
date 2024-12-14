use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn parse_file_to_hmap(filename: &str) -> io::Result<HashMap<u64, Vec<u64>>> {
    let mut hmap: HashMap<u64, Vec<u64>> = HashMap::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let key: u64 = parts[0].parse().unwrap();
            let values: Vec<u64> = parts[1]
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            hmap.insert(key, values);
        }
    }
    Ok(hmap)
}

fn generate_results(numbers: &[u64], functions: &[fn(u64, u64) -> u64]) -> HashSet<u64> {
    if numbers.len() < 2 {
        panic!("The numbers array must contain at least 2 elements");
    }
    let mut results = HashSet::new();

    let mut previous_numbers = vec![numbers[0]];
    for i in 1..numbers.len() {
        let mut next_previous_numbers = vec![];
        for f in functions {
            for previous_number in &previous_numbers {
                next_previous_numbers.push(f(*previous_number, numbers[i]));
                if i == numbers.len() - 1 {
                    results.insert(f(*previous_number, numbers[i]));
                }
            }
        }
        previous_numbers = next_previous_numbers;
    }
    results
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn calibration_result(hmap: &HashMap<u64, Vec<u64>>, functions: &[fn(u64, u64) -> u64],) -> u64 {
    let mut result = 0;
    for (key, values) in hmap {
        let results = generate_results(values, functions);
        if results.contains(&key) {
            result += key;
        }
    }
    result
}

fn main() {
    let hmap = parse_file_to_hmap("inputs\\input.txt").unwrap();
    let result = calibration_result(&hmap, &[add, mul]);
    println!("The calibration result is: {}", result);
    let result = calibration_result(&hmap, &[add, mul, concat]);
    println!("The calibration result with concat is: {}", result);

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_results() {
        let numbers = vec![1, 2, 3, 4];
        let functions = vec![add, mul];
        let results = generate_results(&numbers, &functions);
        let expected_results: HashSet<u64> = [10, 9, 13, 24, 20, 36].iter().cloned().collect();
        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_calibration_result() {
        let hmap = parse_file_to_hmap("inputs\\test.txt").unwrap();
        let result = calibration_result(&hmap, &[add, mul]);
        assert_eq!(result, 3749);
    }
}
