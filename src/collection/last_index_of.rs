/// Returns the index at which the last occurrence of a element is found in the collection.
///
///
/// # Arguments
///
/// * `collection` - The collection iterate over.
///
/// * `item` - The element to find.
///
/// # Returns
///
/// Index of last occurrence of the given element.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!(None, collection::last_index_of(&[1, 2, 3], &0));
///
/// assert_eq!(Some(3), collection::last_index_of(&[1, 2, 3, 2], &2));
///
/// ```

pub fn last_index_of<C: AsRef<[T]>, T: PartialEq + Clone>(
    collection: &C,
    item: &T,
) -> Option<usize> {
    let vector = collection.as_ref();

    for i in (0..vector.len()).rev() {
        if vector[i] == *item {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_index_of() {
        assert_eq!(None, last_index_of(&[1, 2, 3], &0));
        assert_eq!(Some(0), last_index_of(&[1, 2, 3], &1));
        assert_eq!(Some(3), last_index_of(&[1, 2, 3, 2], &2));
    }
}
