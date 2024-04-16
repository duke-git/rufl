/// Remove duplicate elements in collection(array, vector), use PartialEq equality comparisons.
///
/// # Arguments
///
/// * `collection` - The collection to inspect.
///
/// # Returns
///
/// Returns a new duplicate free vector.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!(vec![1, 2, 3], collection::unique([1, 2, 3]));
///
/// assert_eq!(vec![1, 2, 3], collection::unique(vec![1, 2, 2, 3, 3, 3]));
///
/// ```

pub fn unique<C: AsRef<[T]>, T: Clone + PartialEq>(collection: C) -> Vec<T> {
    crate::collection::unique_by(collection, |v1, v2| v1 == v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        assert_eq!(vec![1, 2, 3], unique([1, 2, 3]));
        assert_eq!(vec![1, 2, 3], unique(vec![1, 2, 2, 3, 3, 3]));
    }
}
