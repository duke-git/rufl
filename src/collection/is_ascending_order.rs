/// Checks if all elements are in ascending order within collection.
///
/// # Arguments
///
/// * `collection` - The collection to perform check.
///
/// # Returns
///
/// Returns true if all elements are in ascending order within collection.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!(true, collection::is_ascending_order(&[1, 2, 3]));
///
/// assert_eq!(false, collection::is_ascending_order(&[3, 2, 1]));
/// ```

pub fn is_ascending_order<C: AsRef<[T]>, T: Ord>(collection: &C) -> bool {
    let vec = collection.as_ref();

    for i in 1..vec.len() {
        if &vec[i - 1] > &vec[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ascending_order() {
        assert_eq!(true, is_ascending_order(&[1]));
        assert_eq!(true, is_ascending_order(&[1, 2, 3]));
        assert_eq!(false, is_ascending_order(&[1, 3, 2]));
        assert_eq!(false, is_ascending_order(&[3, 2, 1]));
    }
}
