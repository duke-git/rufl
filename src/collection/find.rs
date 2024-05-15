/// Iterates over elements of collection, returning the first one and its index that pass predicate function.
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
/// Returns the first element and its index which pass the predicate function check.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!(Some((4, 3)), collection::find([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 0));
///
/// assert_eq!(Some((3, 2)), collection::find([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n % 2 != 0, 2));
/// ```

pub fn find<C: AsRef<[T]>, T: Clone>(
    collection: C,
    predicate: impl Fn(&T, usize) -> bool,
    find_from: usize,
) -> Option<(T, usize)> {
    let vec = collection.as_ref();

    if find_from > vec.len() - 1 {
        return None;
    }

    for i in find_from..vec.len() {
        let item = &vec.as_ref()[i];

        if predicate(item, i) {
            return Some((item.clone(), i));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(
            Some((4, 3)),
            find([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 0)
        );

        assert_eq!(
            Some((3, 2)),
            find([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n % 2 != 0, 2)
        );

        assert_eq!(
            None,
            find([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n % 2 != 0, 5)
        );
    }
}
