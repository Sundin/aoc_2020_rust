use std::error::Error;

mod files;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        let day = files::to_number(&day);

        Ok(Config {
            filename,
            day,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error rather than panic
    let contents = files::get_file_contents(config.filename)?;
    
    match config.day {
        1 => day1(files::as_integers(&contents)),
        2 => day2(&contents),
        3 => day3(&contents),
        4 => day4(&contents),
        5 => day5(&contents),
        6 => day6(&contents),
        7 => day7(&contents),
        8 => day8(&contents),
        9 => day9(files::as_integers(&contents)),
        _ => { println!("Not implemented") }
    }
    
    Ok(())
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
    let answer = day3::part_2(&contents);
    println!("Answer for day 3 part 2: {}", answer);
}

fn day4(contents: &str) {
    let answer = day4::part_1(&contents);
    println!("Answer for day 4 part 1: {}", answer);
    let answer = day4::part_2(&contents);
    println!("Answer for day 4 part 2: {}", answer);
}

fn day5(contents: &str) {
    let answer = day5::part_1(&contents);
    println!("Answer for day 5 part 1: {}", answer);
    let answer = day5::part_2(&contents);
    println!("Answer for day 5 part 2: {}", answer);
}

fn day6(contents: &str) {
    let answer = day6::part_1(&contents);
    println!("Answer for day 6 part 1: {}", answer);
    let answer = day6::part_2(&contents);
    println!("Answer for day 6 part 2: {}", answer);
}

fn day7(contents: &str) {
    let answer = day7::part_1(&contents);
    println!("Answer for day 7 part 1: {}", answer);
    let answer = day7::part_2(&contents);
    println!("Answer for day 7 part 2: {}", answer);
}

fn day8(contents: &str) {
    let answer = day8::part_1(&contents);
    println!("Answer for day 8 part 1: {}", answer);
    let answer = day8::part_2(&contents);
    println!("Answer for day 8 part 2: {}", answer);
}

fn day9(contents: Vec<i32>) {
    let answer = day9::part_1(&contents, 25);
    println!("Answer for day 9 part 1: {}", answer);
    let answer = day9::part_2(&contents);
    println!("Answer for day 9 part 2: {}", answer);
}
