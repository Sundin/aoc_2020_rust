use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;


pub fn part_1(input: &str) -> usize {
    bags_that_contain(input, "shiny gold").len()
}

fn bags_that_contain(input: &str, color: &str) -> HashSet<String> {
    let mut found_bags: HashSet<String> = HashSet::new();
    for line in input.lines() {
        let (found, found_color) = bag_contains_color(&line, &color);
        if found {
            found_bags.insert(found_color.to_owned());
            for bag in bags_that_contain(&input, &found_color) {
                found_bags.insert(bag);
            }
        }
    }
    found_bags
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
            return (true, bag_color.to_string());
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
