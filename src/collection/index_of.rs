/// Returns the index at which the first occurrence of a element is found in the collection.
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
/// Index of first occurrence of the given element.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(None, collection::index_of(&[1, 2, 3], &0));
///
/// assert_eq!(Some(1), collection::index_of(&[1, 2, 3, 2], &2));
/// 
/// ```

pub fn index_of<C: AsRef<[T]>, T: PartialEq+Clone>(collection: &C, item: &T) -> Option<usize> {
    for (i, v) in collection.as_ref().to_vec().iter().enumerate() {
        if *v == *item {
            return Some(i)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_of() {
        assert_eq!(None, index_of(&[1, 2, 3], &0));
        assert_eq!(Some(0), index_of(&[1, 2, 3], &1));
        assert_eq!(Some(1), index_of(&[1, 2, 3, 2], &2));
    }
}
