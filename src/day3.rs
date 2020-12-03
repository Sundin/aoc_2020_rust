use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Row {
    pub trees: Vec<bool>, 
}

impl FromStr for Row {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut trees: Vec<bool> = vec![];
        for c in input.chars() {
            trees.push(c == '#');
        }

        Ok(Row { trees: trees })
    }
}

fn has_tree_at(row: Row, x: usize) -> bool {
    let length = row.trees.len();
    let modulus = x / length;
    let remainder = x - modulus * length;
    row.trees[remainder]
}

pub fn part_1(input: &str) -> i32 {
    let mut count = 0;
    for (x, line) in input.lines().enumerate() {
        let row = Row::from_str(line).unwrap();
        if has_tree_at(row, x*3) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let row = Row::from_str("..##.").unwrap();
        let expected = Row{ trees: vec![false,false,true,true,false] };
        assert_eq!(expected, row);
    }

    #[test]
    fn test_tree_at() {
        let row = Row::from_str("..##.").unwrap();
        assert_eq!(false, has_tree_at(row, 0));
        let row = Row::from_str("..##.").unwrap();
        assert_eq!(true, has_tree_at(row, 2));
        let row = Row::from_str("..##.").unwrap();
        assert_eq!(true, has_tree_at(row, 2));
        let row = Row::from_str("..##.").unwrap();
        assert_eq!(false, has_tree_at(row, 5));
    }
}
