use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    0
}

pub fn part_2(input: &str) -> i32 {
    0
}

fn count_unique_letters(input: &str) -> usize {
    let mut letters: HashSet<char> = HashSet::new();
    for c in input.chars() {
        if c == ' ' || c == '\n' {
            continue;
        }
        letters.insert(c);
    }
    letters.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day6.txt".to_owned()).unwrap();
        assert_eq!(0, part_1(&contents))
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day6.txt".to_owned()).unwrap();
        assert_eq!(0, part_2(&contents))
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
