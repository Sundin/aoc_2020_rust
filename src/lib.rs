use std::error::Error;
use std::fs;

mod expense;

pub struct Config {
    pub filename: String,
    pub day: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let day = args[1].clone();
        let filename = format!("input/day{}.txt", day);
        let day = to_number(&day);

        Ok(Config {
            filename,
            day,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error rather than panic
    let contents = fs::read_to_string(config.filename)?;
    let numbers = contents.lines().map(|x| to_number(x)).collect::<Vec<i32>>();
    
    match config.day {
        1 => day1(numbers),
        _ => { println!("Not implemented") }
    }
    
    Ok(())
}

fn to_number(line: &str) -> i32 {
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

fn day1(contents: Vec<i32>) {
    let answer = expense::multiply_two_entries_with_sum(&contents, 0);
    println!("Answer for day 1 part 1: {}", answer);
    let answer = expense::multiply_three_entries_with_sum(&contents, 0);
    println!("Answer for day 1 part 2: {}", answer);
}
