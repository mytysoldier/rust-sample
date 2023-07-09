pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(add(10, 0), 10);
        assert_eq!(add(0, 10), 10);
        assert_eq!(add(10, 10), 20);
    }
    #[test]
    fn test_add_zero() {
        assert_eq!(0, add(0, 0));
    }
}
