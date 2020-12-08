use lazy_static::lazy_static;
use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    for line in input.lines() {
        bag_contains_color(&line, "shiny gold");
    }
    0
}

fn contains_bag(input: &str, color: &str) {
    for line in input.lines() {
        bag_contains_color(&line, "shiny gold");
    }
}

fn bag_contains_color(input: &str, color: &str) -> (bool, String) {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"([a-z ]+) bag"
            ).unwrap();
    }
    let bag_color = REGEX.captures_iter(input).nth(0).unwrap();
    let bag_color = &bag_color[1];

    for (i, cap) in REGEX.captures_iter(input).enumerate() {
        let current_color = &cap[1].trim();
        if i == 0 {
            continue;
        } else if current_color.eq(&color) {
            println!("{} contains {}", bag_color, current_color);
        }
    }
    (false, String::new())
}

pub fn part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day7.txt".to_owned()).unwrap();
        assert_eq!(4, part_1(&contents))
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day7.txt".to_owned()).unwrap();
        assert_eq!(0, part_2(&contents))
    }
}
