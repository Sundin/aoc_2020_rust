use std::error::Error;
use std::fs;

mod day1;
mod day2;
mod day3;

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
    
    match config.day {
        1 => day1(as_integers(&contents)),
        2 => day2(&contents),
        3 => day3(&contents),
        _ => { println!("Not implemented") }
    }
    
    Ok(())
}

fn as_integers(contents: &str) -> Vec<i32> {
    contents.lines().map(|x| to_number(x)).collect::<Vec<i32>>()
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
    let answer = day1::multiply_two_entries_with_sum(&contents, 2020);
    println!("Answer for day 1 part 1: {}", answer);
    let answer = day1::multiply_three_entries_with_sum(&contents, 2020);
    println!("Answer for day 1 part 2: {}", answer);
}

fn day2(contents: &str) {
    let answer = day2::count_valid_password(&contents);
    println!("Answer for day 2 part 1: {}", answer);
    let answer = day2::count_valid_occurences(&contents);
    println!("Answer for day 2 part 2: {}", answer);
}

fn day3(contents: &str) {
    let answer = day3::part_1(&contents);
    println!("Answer for day 3 part 1: {}", answer);
}
