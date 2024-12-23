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

fn get_reports(input_file: String) -> Vec<Vec<u32>> {
    let mut reports: Vec<Vec<u32>> = Vec::new();
    for line in input_file.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Erreur lors du parsing"))
            .collect();
        reports.push(report);
    }
    reports
}

fn is_increasing(report: &Vec<u32>) -> (bool, Option<usize>) {
    for (i, window) in report.windows(2).enumerate() {
        if window[0] >= window[1] {
            return (false, Some(i));
        }
    }
    (true, None)
}

fn is_decreasing(report: &Vec<u32>) -> (bool, Option<usize>) {
    for (i, window) in report.windows(2).enumerate() {
        if window[0] <= window[1] {
            return (false, Some(i));
        }
    }
    (true, None)
}

fn is_difference_at_most_three(report: &Vec<u32>) -> (bool, Option<usize>) {
    for (i, window) in report.windows(2).enumerate() {
        let diff = window[0].abs_diff(window[1]);
        if diff >= 4 {
            return (false, Some(i));
        }
    }
    (true, None)
}

fn report_is_safe(report: &Vec<u32>) -> bool {
    if !(is_increasing(report).0 || is_decreasing(report).0) {
        return false;
    }
    if !is_difference_at_most_three(report).0 {
        return false;
    }
    true
}

fn try_remove(report: &Vec<u32>, index: usize) -> bool {
    for &offset in &[0, 1] {
        let removal_index = index + offset;
        if removal_index < report.len() {
            let mut new_report = report.clone();
            new_report.remove(removal_index);
            if report_is_safe(&new_report) {
                return true;
            }
        }
    }
    false
}

fn report_is_almost_safe(report: &Vec<u32>) -> bool {
    if report_is_safe(report) {
        return true;
    }

    if let (false, Some(index)) = is_increasing(report) {
        if try_remove(report, index) {
            return true;
        }
    }

    if let (false, Some(index)) = is_decreasing(report) {
        if try_remove(report, index) {
            return true;
        }
    }

    if let (false, Some(index)) = is_difference_at_most_three(report) {
        if try_remove(report, index) {
            return true;
        }
    }

    false
}

fn count_safe_reports<F>(reports: &Vec<Vec<u32>>, check_fn: F) -> u32
where
    F: Fn(&Vec<u32>) -> bool,
{
    let mut nb_safe: u32 = 0;
    for report in reports {
        if check_fn(report) {
            nb_safe += 1;
        }
    }
    nb_safe
}

fn main() {
    let contents = match get_aoc_input("input.txt") {  // 'match' handles the result of file_parser
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return;
        },
    };

    let reports = get_reports(contents.to_owned());
    let nb_safe_reports = count_safe_reports(&reports, report_is_safe);
    let nb_safe_reports_2 = count_safe_reports(&reports, report_is_almost_safe);
    println!("The total number of safe reports is {}", nb_safe_reports);
    println!("The new total number of safe reports is {}", nb_safe_reports_2);

}
