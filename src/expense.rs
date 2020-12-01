pub fn find_entries_with_sum(contents: &Vec<i32>, sum: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_rounds() {
        let v: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(0, find_entries_with_sum(&v, 0));
    }
}