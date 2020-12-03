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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let row = Row::from_str("..##.").unwrap();
        let expected = Row{ trees: vec![false,false,true,true,false] };
        assert_eq!(expected, row);
    }
}
