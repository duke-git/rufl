/// Iterates over elements of collection, returning the last one and its index that pass predicate function.
///
/// * predicate function signature: ```fn(item: T, index: usize) -> bool```
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `predicate` - The check function invoked per iteration.
///
/// * `find_from` - The position to start find.
///
///
/// # Returns
///
/// Returns the last element and its index which pass the predicate function check.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(Some((4, 3)), collection::find_last([1, 2, 3, 4, 5], &|n: i32, _i: usize| n > 3, 3));
///
/// assert_eq!(None, collection::find_last([1, 2, 3, 4, 5], &|n: i32, _i: usize| n > 3, 5));
/// ```

pub fn find_last<C: AsRef<[T]>, T: Copy>(
    collection: C,
    predicate: &dyn Fn(T, usize) -> bool,
    find_from: usize,
) -> Option<(T, usize)> {
    if find_from > collection.as_ref().len() - 1 {
        return None;
    }

    let mut find_end = 1;

    if find_from > 0 {
        find_end = find_from;
    }

    for i in 0..find_end {
        let val = &collection.as_ref()[find_from - i];
        if predicate(*val, find_from - i) {
            return Some((*val, find_from - i));
        }
    }

    None
}

mod tests {
    use super::*;

    #[test]
    fn test_find_last() {
        assert_eq!(
            Some((4, 3)),
            find_last([1, 2, 3, 4, 5], &|n: i32, _i: usize| n > 3, 3)
        );

        assert_eq!(
            Some((5, 4)),
            find_last([1, 2, 3, 4, 5], &|n: i32, _i: usize| n > 3, 4)
        );

        assert_eq!(
            None,
            find_last([1, 2, 3, 4, 5], &|n: i32, _i: usize| n > 3, 5)
        );

        assert_eq!(
            Some((5, 4)),
            find_last([1, 2, 3, 4, 5], &|n: i32, _i: usize| n % 2 != 0, 4)
        );
    }
}
