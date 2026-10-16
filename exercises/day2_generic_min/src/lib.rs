pub fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(min(10, 20), 10);
        assert_eq!(min(50, 30), 30);
        assert_eq!(min("apple", "banana"), "apple");
    }
}
