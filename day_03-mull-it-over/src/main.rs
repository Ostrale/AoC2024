use std::fs;
use std::io::Result;
use std::path::Path;
use std::env;
use regex::Regex;

#[derive(Debug)]
struct MulInstruction {
    full_text: String,
    value1: u32,
    value2: u32,
}

impl MulInstruction {
    fn new(full_text: String, value1: u32, value2: u32) -> Self {
        MulInstruction {
            full_text,
            value1,
            value2,
        }
    }
}

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

fn find_mul_instruction(content: &String, with_opt: bool) -> Vec<MulInstruction> {
    let mut mul_instructions = vec![];
    let re = Regex::new(r"(?m)mul\((\d+)\,(\d+)\)").unwrap();
    for capture in re.captures_iter(&content) {
        let mul: MulInstruction = MulInstruction::new(
            capture.get(0).unwrap().as_str().to_owned(),
            capture.get(1).unwrap().as_str().parse().unwrap(),
            capture.get(2).unwrap().as_str().parse().unwrap(),
        );
        if !with_opt {
            mul_instructions.push(mul);
        } else {
            let subline = &content[..capture.get(0).unwrap().start()];
            let re_subline = Regex::new(r"(^|don't\(\)|do\(\))").unwrap();
            let last_capture = re_subline.captures_iter(subline).last();
            let option = last_capture.unwrap().get(0).unwrap().as_str().to_owned();
            if option != "don't()" {
                mul_instructions.push(mul);
            }
        }
    }
    mul_instructions
}

fn result (mul_instructions: &Vec<MulInstruction>) -> u32 {
    let mut res:u32 = 0;
    for mul in mul_instructions {
        res += mul.value1 * mul.value2;
    }
    res
}

fn main() {
    let contents = match get_aoc_input("input.txt") {  // 'match' handles the result of file_parser
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return;
        },
    };
    let mul_instructions = find_mul_instruction(&contents, false);
    let result_1:u32 = result(&mul_instructions);
    println!("The total is {}", result_1);
    let mul_instructions_2 = find_mul_instruction(&contents, true);
    let result_2:u32 = result(&mul_instructions_2);
    println!("The total is {}", result_2);
}
