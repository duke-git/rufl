/// Checks if all elements are sorted(ascending or descending order) within collection.
///
/// # Arguments
///
/// * `collection` - The collection to perform check.
///
/// # Returns
///
/// Returns true if all elements are sorted within collection.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(true, collection::is_sorted(&[1, 2, 3]));
///
/// assert_eq!(true, collection::is_sorted(&[3, 2, 1]));
///
/// assert_eq!(false, collection::is_sorted(&[1, 3, 2]));
///
/// ```

pub fn is_sorted<C: AsRef<[T]>, T: Ord>(collection: &C) -> bool {
    crate::collection::is_ascending_order(collection)
        || crate::collection::is_descending_order(collection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert_eq!(true, is_sorted(&[1]));
        assert_eq!(true, is_sorted(&[1, 2, 3]));
        assert_eq!(true, is_sorted(&[3, 2, 1]));
        assert_eq!(true, is_sorted(&["c", "b", "a"]));

        assert_eq!(false, is_sorted(&[3, 1, 2]));
        assert_eq!(false, is_sorted(&[1, 3, 2]));
    }
}
