/// Returns the number of occurrences of the given element in the collection.
///
///
/// # Arguments
///
/// * `collection` - The collection iterate over.
///
/// * `item` - The element to count.
///
/// # Returns
///
/// Returns number of occurrences of the given element.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(1, collection::count(&[1, 2, 3], 1));
///
/// assert_eq!(0, collection::count(&[1, 2, 3], 4));
/// ```

pub fn count<C: AsRef<[T]>, T: PartialEq>(collection: &C, item: T) -> usize {
    let mut count = 0;
    let vec = collection.as_ref();

    for i in 0..vec.len() {
        if &vec[i] == &item {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(1, count(&[1, 2, 3], 1));
        assert_eq!(0, count(&[1, 2, 3], 4));
    }
}
