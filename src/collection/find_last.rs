/// Iterates over elements of collection, returning the last one and its index that pass predicate function.
///
/// * predicate function signature: ```fn(item: &T, index: usize) -> bool```
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
/// use rufl::collection;
///
/// assert_eq!(Some((4, 3)), collection::find_last([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 3));
///
/// assert_eq!(None, collection::find_last([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 5));
/// ```

pub fn find_last<C: AsRef<[T]>, T: Clone>(
    collection: C,
    predicate: impl Fn(&T, usize) -> bool,
    find_from: usize,
) -> Option<(T, usize)> {
    let vec = collection.as_ref();

    if find_from > vec.len() - 1 {
        return None;
    }

    let mut find_end = 1;

    if find_from > 0 {
        find_end = find_from;
    }

    for i in 0..find_end {
        let item = &vec[find_from - i];
        if predicate(item, find_from - i) {
            return Some((item.clone(), find_from - i));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_last() {
        assert_eq!(
            Some((4, 3)),
            find_last([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 3)
        );

        assert_eq!(
            Some((5, 4)),
            find_last([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 4)
        );

        assert_eq!(
            None,
            find_last([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 5)
        );

        assert_eq!(
            Some((5, 4)),
            find_last([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n % 2 != 0, 4)
        );
    }
}
