use std::fs;
use std::env;

use day_09::{Disk, compact_disk, compact_disk_v2}; 

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let input_file_path = current_dir.join("inputs/input.txt");
    let input = fs::read_to_string(input_file_path).expect("Failed to read input file");

    let mut disk = Disk::from_string(input.clone().as_str());
    compact_disk(&mut disk);
    println!("{}", disk.checksum());

    let mut disk = Disk::from_string(input.as_str());
    compact_disk_v2(&mut disk);
    println!("{}", disk.checksum());
}
