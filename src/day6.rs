use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    let mut count = 0;
    let mut group = String::new();
    for line in input.lines() {
        if line == "" {
            count += count_unique_letters(&group);
            group = String::new();
        } else {
            group = String::new() + &group + line + " ";
        }
    }
    count += count_unique_letters(&group);
    count
}

pub fn part_2(input: &str) -> i32 {
    let mut count = 0;
    let mut group: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line == "" {
            let first = group.first().unwrap();
            for c in first.chars() {
                if all_contains_char(&group, c) {
                    count += 1;
                }
            }
            group = Vec::new();
        } else {
            group.push(line);
        }
    }

    // TODO: Must find a nicer way of getting the last element too...
    let first = group.first().unwrap();
    for c in first.chars() {
        if all_contains_char(&group, c) {
            count += 1;
        }
    }

    count
}

fn all_contains_char(group: &Vec<&str>, c: char) -> bool {
    for person in group {
        if !person.contains(c) {
            return false;
        }
    }
    true
}

fn count_unique_letters(input: &str) -> i32 {
    let mut letters: HashSet<char> = HashSet::new();
    for c in input.chars() {
        if c == ' ' || c == '\n' {
            continue;
        }
        letters.insert(c);
    }
    letters.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day6.txt".to_owned()).unwrap();
        assert_eq!(11, part_1(&contents))
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day6.txt".to_owned()).unwrap();
        assert_eq!(6, part_2(&contents))
    }

    #[test]
    fn test_letters() {
        assert_eq!(3, count_unique_letters("abc"));
        assert_eq!(3, count_unique_letters("aabcb"));
        assert_eq!(3, count_unique_letters("
        aab
        cb"));
    }
}
