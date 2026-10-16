use std::collections::HashMap;
use std::hash::Hash;

pub fn count_frequencies<T, I>(items: I) -> HashMap<T, usize>
where
    T: Eq + Hash,
    I: IntoIterator<Item = T>,
{
    let mut map = HashMap::new();
    for item in items {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let words = vec!["rust", "fast", "safe", "rust", "rust"];
        let counts = count_frequencies(words);
        assert_eq!(counts.get("rust"), Some(&3));
        assert_eq!(counts.get("fast"), Some(&1));
    }
}
