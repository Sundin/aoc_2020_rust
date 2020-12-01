use std::error::Error;
use std::fs;

mod expense;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let day = args[1].clone();
        let filename = format!("input/day{}.txt", day);

        Ok(Config {
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error rather than panic
    let contents = fs::read_to_string(config.filename)?;
    let numbers = contents.lines().map(|x| to_number(x)).collect::<Vec<i32>>();
    

    day1(numbers);

    Ok(())
}

fn day1(contents: Vec<i32>) {
    let answer = expense::find_entries_with_sum(&contents, 0);
    println!("Answer for day 1 part 1: {}", answer);
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
