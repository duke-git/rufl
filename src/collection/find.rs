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
/// Returns the first element and its index which pass the predicate function check.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(Some((4, 3)), collection::find([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n > 3, 0));
///
/// assert_eq!(Some((3, 2)), collection::find([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n % 2 != 0, 2));
/// ```

pub fn find<C: AsRef<[T]>, T: Copy>(
    collection: C,
    predicate: &dyn Fn(&T, usize) -> bool,
    find_from: usize,
) -> Option<(T, usize)> {
    if find_from > collection.as_ref().len() - 1 {
        return None;
    }

    for i in find_from..collection.as_ref().len() {
        let val = &collection.as_ref()[i];
        if predicate(val, i) {
            return Some((*val, i));
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
