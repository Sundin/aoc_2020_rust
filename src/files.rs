use std::io::Error;
use std::fs;

pub fn get_file_contents(filename: String) -> Result<String, Error>{
    fs::read_to_string(filename)
}

pub fn as_integers(contents: &str) -> Vec<i32> {
    contents.lines().map(|x| to_number(x)).collect::<Vec<i32>>()
}

pub fn to_number(line: &str) -> i32 {
    let line: i32 = match line
        .trim()
        .parse() {
            Ok(num) => num,
            // _ = match all Err values (place for proper error handling)
            Err(_) => {
                // TODO: error handling
                println!("Please type a number!");
                0
            }
        };
    line
}
