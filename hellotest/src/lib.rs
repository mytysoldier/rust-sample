pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32,
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
    fn test_equal_instance() {
        let mut a = Person {
            id: 100,
            name: "masu".to_string(),
            age: 50,
        };
        let mut b = Person {
            id: 100,
            name: "masu".to_string(),
            age: 50,
        };
        let mut c = Person {
            id: 200,
            name: "kato".to_string(),
            age: 40,
        };
        assert_eq!(a, a);
        assert_eq!(a, b);
        let x = &a;
        assert_eq!(a, *x);
    }
}
