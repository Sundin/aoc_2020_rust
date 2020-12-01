pub fn find_entries_with_sum(contents: &Vec<i32>, sum: i32) -> (i32, i32) {
    let length = contents.len();
    for x in 0..length {
        for y in 0..length {
            if contents[x] + contents[y] == 2020 {
                println!("{} + {} = 2020", contents[x], contents[y]);
                return (contents[x], contents[y]);
            }
        }
    }
    (0,0)
}

pub fn multiply_entries_with_sum(contents: &Vec<i32>, sum: i32) -> i32 {
    let (x, y) = find_entries_with_sum(&contents, sum);
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_entries() {
        let v: Vec<i32> = vec![1721,979,366,299,675,1456];
        assert_eq!((1721, 299), find_entries_with_sum(&v, 0));
    }

    #[test]
    fn test_multiply_entries() {
        let v: Vec<i32> = vec![1721,979,366,299,675,1456];
        assert_eq!(514579, multiply_entries_with_sum(&v, 0));
    }
}